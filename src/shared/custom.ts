import type { HfModel } from './higgsfield'

/**
 * Image models served by the Custom provider: any OpenAI-compatible image
 * endpoint (settings.customBaseUrl + the "custom" secret key, the same pair
 * the Custom LLM provider uses). The id prefix "custom:" is stripped before
 * the model name is sent to the endpoint, so the picker entry IS the model.
 * Proven against chatgpt2api + gpt-image-2.
 */
export const CUSTOM_IMAGE_MODELS: HfModel[] = [
  {
    id: 'custom:gpt-image-2',
    name: 'GPT Image 2 (Custom)',
    family: 'Endpoint Custom',
    kind: 'image',
    input: {
      field: 'image',
      mode: 'reference',
      max: 3
    },
    required: ['prompt'],
    props: {
      prompt: {
        type: 'string',
        maxLength: 4000
      },
      aspect_ratio: {
        type: 'string',
        enum: ['16:9', '9:16', '1:1', '4:3', '3:4'],
        default: '16:9'
      },
      resolution: {
        type: 'string',
        enum: ['1k', '2k'],
        default: '2k'
      }
    },
    durations: null,
    tags: ['Endpoint Custom', 'Referensi', 'GPT Image']
  },
  {
    id: 'custom:gpt-image-1.5',
    name: 'GPT Image 1.5 (Custom)',
    family: 'Endpoint Custom',
    kind: 'image',
    input: {
      field: 'image',
      mode: 'reference',
      max: 3
    },
    required: ['prompt'],
    props: {
      prompt: {
        type: 'string',
        maxLength: 4000
      },
      aspect_ratio: {
        type: 'string',
        enum: ['16:9', '9:16', '1:1', '4:3', '3:4'],
        default: '16:9'
      },
      resolution: {
        type: 'string',
        enum: ['1k', '2k'],
        default: '2k'
      }
    },
    durations: null,
    tags: ['Endpoint Custom', 'Referensi', 'GPT Image']
  }
]
