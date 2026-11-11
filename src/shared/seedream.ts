import type { HfModel } from './higgsfield'

/**
 * Image models served by the Seedream provider: BytePlus ModelArk
 * (settings.seedreamBaseUrl + the "seedream" secret key — a SEPARATE pair
 * from the Custom LLM provider, so script stays on chatgpt2api while
 * images can use Seedream). The id prefix "seedream:" is stripped before
 * the model name is sent to the endpoint, so the picker entry IS the model.
 * Endpoint: https://ark.ap-southeast.bytepluses.com/api/v3
 * Proven against ModelArk + seedream-4-5-251128.
 */
export const SEEDREAM_IMAGE_MODELS: HfModel[] = [
  {
    id: 'seedream:seedream-4-5-251128',
    name: 'Seedream 4.5',
    family: 'Seedream',
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
      }
    },
    tags: ['Seedream', 'Referensi', 'BytePlus'],
    durations: null
  },
  {
    id: 'seedream:dola-seedream-5-0-flash-260915',
    name: 'Seedream 5.0 Flash',
    family: 'Seedream',
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
      }
    },
    tags: ['Seedream', 'Referensi', 'BytePlus'],
    durations: null
  },
  {
    id: 'seedream:dola-seedream-5-0-pro-260628',
    name: 'Seedream 5.0 Pro',
    family: 'Seedream',
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
      }
    },
    tags: ['Seedream', 'Referensi', 'BytePlus'],
    durations: null
  }
]

export const DEFAULT_SEEDREAM_BASE_URL = 'https://ark.ap-southeast.bytepluses.com/api/v3'
