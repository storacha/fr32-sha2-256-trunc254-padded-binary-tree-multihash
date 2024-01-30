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
// @ts-expect-error
let bytecode = (await import("../gen/wasm_bg.wasm")).default

await load(bytecode)

export { create }
