import load, { create } from "../gen/wasm.js"
import {
  code,
  CODE_LENGTH,
  HEIGHT_SIZE,
  ROOT_SIZE,
  MAX_SIZE,
} from "./constant.js"
export * from "./type.js"
export { code, CODE_LENGTH, HEIGHT_SIZE, ROOT_SIZE, MAX_SIZE }

// load bytecode in Cloudflare Workers as wasm import
// all other paths are disallowed by embedder
let bytecode
// https://developers.cloudflare.com/workers/runtime-apis/web-standards/#navigatoruseragent
export const CLOUDFLARE_WORKERS_NAVIGATOR = "Cloudflare-Workers"
/* c8 ignore start */
try {
  if (globalThis.navigator?.userAgent === CLOUDFLARE_WORKERS_NAVIGATOR) {
    // playwright tries to bundle imported file when it sees import with
    // string literal. We workaround by using variable to import.
    const wasm = "../gen/wasm_bg.wasm"
    bytecode = (await import(wasm)).default
  }
} catch {}
/* c8 ignore stop */

await load(bytecode)

export { create }
