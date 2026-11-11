//! Seedream image provider: BytePlus ModelArk (ByteDance) image generation.
//!
//! The Ark image endpoint is OpenAI-compatible in shape: POST
//! {base}/images/generations JSON {model, prompt, size} (with refs,
//! multipart image[] like /images/edits). The response carries
//! data[0].url (a download link, valid 24h) or data[0].b64_json.
//! So generation reuses `custom_visual::generate_image` (generic) —
//! Seedream only needs its OWN base URL + key, separate from the
//! Custom LLM provider (which drives script via chatgpt2api).
//!
//! Endpoint (region ap-southeast-1): https://ark.ap-southeast.bytepluses.com/api/v3
//! Auth: Bearer <ModelArk API key> (console.byteplus.com -> ModelArk -> API Key).
//! The model must be ACTIVATED in the ModelArk console first, or the
//! endpoint answers 404 ModelNotOpen.

use serde_json::json;

use crate::services::generation::{HfInput, HfModel};

/// Default Seedream (ModelArk) base URL, region ap-southeast-1.
pub const DEFAULT_SEEDREAM_BASE_URL: &str = "https://ark.ap-southeast.bytepluses.com/api/v3";

/// Image models the Seedream provider offers in the model picker.
/// The id prefix "seedream:" is stripped before sending the model name.
pub fn seedream_image_models() -> Vec<HfModel> {
    let mk = |id: &str, name: &str| HfModel {
        id: id.to_string(),
        name: name.to_string(),
        family: "Seedream".to_string(),
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
            "Seedream".to_string(),
            "Referensi".to_string(),
            "BytePlus".to_string(),
        ],
    };
    vec![
        mk("seedream:seedream-4-5-251128", "Seedream 4.5"),
        mk("seedream:dola-seedream-5-0-flash-260915", "Seedream 5.0 Flash"),
        mk("seedream:dola-seedream-5-0-pro-260628", "Seedream 5.0 Pro"),
    ]
}
