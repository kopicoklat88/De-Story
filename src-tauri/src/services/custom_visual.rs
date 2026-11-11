//! Custom visual provider: any OpenAI-compatible image endpoint.
//!
//! Mirrors the Custom provider in SisiStory (Electron), proven against
//! chatgpt2api + gpt-image-2. No refs -> POST {base}/images/generations (JSON).
//! With refs -> POST {base}/images/edits (multipart, image[] parts).
//! The response carries data[0].b64_json (base64 PNG) or data[0].url.
//!
//! The base URL and API key are the SAME settings the Custom LLM provider
//! uses (settings.customBaseUrl + secret provider "custom"), so one ChatGPT
//! key drives both script and images.

use std::time::Duration;

use base64::prelude::*;
use serde_json::{json, Value};
use tokio_util::sync::CancellationToken;

use crate::services::flow2api::sanitize_url;
use crate::services::generation::{HfInput, HfModel};
use crate::services::http::download_to;

/// Image models the Custom provider offers in the model picker.
/// The id prefix "custom:" is stripped before sending the model name.
pub fn custom_image_models() -> Vec<HfModel> {
    let mk = |id: &str, name: &str| HfModel {
        id: id.to_string(),
        name: name.to_string(),
        family: "Endpoint Custom".to_string(),
        kind: "image".to_string(),
        input: HfInput {
            field: Some("image".to_string()),
            mode: "reference".to_string(),
            max: 3,
        },
        required: vec!["prompt".to_string()],
        props: {
            let mut p = std::collections::HashMap::new();
            p.insert(
                "prompt".to_string(),
                json!({ "type": "string", "maxLength": 4000 }),
            );
            p.insert(
                "aspect_ratio".to_string(),
                json!({ "type": "string", "enum": ["16:9", "9:16", "1:1", "4:3", "3:4"], "default": "16:9" }),
            );
            p.insert(
                "resolution".to_string(),
                json!({ "type": "string", "enum": ["1k", "2k"], "default": "2k" }),
            );
            p
        },
        durations: None,
        tags: vec![
            "Endpoint Custom".to_string(),
            "Referensi".to_string(),
            "GPT Image".to_string(),
        ],
    };
    vec![
        mk("custom:gpt-image-2", "GPT Image 2 (Custom)"),
        mk("custom:gpt-image-1.5", "GPT Image 1.5 (Custom)"),
    ]
}

/// Size the endpoint accepts for an aspect ratio (same map as SisiStory).
fn custom_size(aspect_ratio: &str) -> &'static str {
    match aspect_ratio {
        "9:16" => "1024x1536",
        "16:9" => "1536x1024",
        _ => "1024x1024",
    }
}

fn friendly_error(status: reqwest::StatusCode, body: &str) -> String {
    let mut message = String::new();
    if let Ok(parsed) = serde_json::from_str::<Value>(body) {
        if let Some(m) = parsed.get("error").and_then(|e| e.get("message")).and_then(|m| m.as_str()) {
            message = m.to_string();
        }
    }
    if status.as_u16() == 401 {
        return "Kunci custom ditolak. Periksa kunci + alamat endpoint custom di Pengaturan.".to_string();
    }
    if status.as_u16() == 429 || message.to_lowercase().contains("insufficient_quota") || message.to_lowercase().contains("billing") {
        return "Endpoint custom menolak permintaan (kuota atau saldo habis?). Cek penyedia API kamu.".to_string();
    }
    if !message.is_empty() {
        return format!("Endpoint custom: {message}");
    }
    format!("Endpoint custom menjawab dengan status {status}.")
}

/// Generate one image through an OpenAI-compatible endpoint.
/// Returns the raw image bytes (PNG/JPEG/WebP, as the endpoint sent them).
pub async fn generate_image(
    base_url: &str,
    key: Option<&str>,
    model_id: &str,
    prompt: &str,
    aspect_ratio: &str,
    ref_images: &[Vec<u8>],
    token: &CancellationToken,
) -> Result<Vec<u8>, String> {
    let base = sanitize_url(base_url);
    let model = model_id.trim_start_matches("custom:");
    let size = custom_size(aspect_ratio);

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(300))
        .build()
        .map_err(|e| e.to_string())?;

    let mut req = if ref_images.is_empty() {
        client
            .post(format!("{base}/images/generations"))
            .json(&json!({
                "prompt": prompt,
                "model": model,
                "size": size,
                "n": 1
            }))
    } else {
        let mut form = reqwest::multipart::Form::new();
        for (i, img) in ref_images.iter().enumerate() {
            let part = reqwest::multipart::Part::bytes(img.clone())
                .file_name(format!("ref-{}.png", i + 1))
                .mime_str("image/png")
                .map_err(|e| e.to_string())?;
            form = form.part("image[]", part);
        }
        form = form
            .text("prompt", prompt.to_string())
            .text("model", model.to_string())
            .text("size", size.to_string());
        client.post(format!("{base}/images/edits")).multipart(form)
    };

    if let Some(k) = key {
        if !k.trim().is_empty() {
            req = req.header("Authorization", format!("Bearer {}", k.trim()));
        }
    }

    let response = tokio::select! {
        _ = token.cancelled() => return Err("Pembuatan gambar dibatalkan".into()),
        res = req.send() => res.map_err(|e| format!("Koneksi ke endpoint custom gagal: {e}"))?,
    };

    let status = response.status();
    let body_text = tokio::select! {
        _ = token.cancelled() => return Err("Pembuatan gambar dibatalkan".into()),
        t = response.text() => t.unwrap_or_default(),
    };

    if !status.is_success() {
        return Err(friendly_error(status, &body_text));
    }

    let parsed: Value = serde_json::from_str(&body_text)
        .map_err(|_| "Endpoint custom mengembalikan respons yang bukan JSON.".to_string())?;
    let first = parsed
        .get("data")
        .and_then(|d| d.as_array())
        .and_then(|a| a.first())
        .cloned()
        .unwrap_or(Value::Null);

    if let Some(b64) = first.get("b64_json").and_then(|b| b.as_str()) {
        let bytes = BASE64_STANDARD
            .decode(b64)
            .map_err(|_| "Endpoint custom mengembalikan gambar base64 yang rusak.".to_string())?;
        return Ok(bytes);
    }
    // Some compatible endpoints answer with a hosted URL instead of base64.
    if let Some(url) = first.get("url").and_then(|u| u.as_str()) {
        let (bytes, _ctype) = download_to(url).await?;
        return Ok(bytes);
    }
    Err("Endpoint custom tidak mengembalikan gambar.".to_string())
}
