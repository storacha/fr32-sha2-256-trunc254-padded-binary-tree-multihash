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

// load bytecode with import
// there are runtimes like cloudflare workers where
// all other paths are disallowed by embedder
// @ts-expect-error
import bytecode from "../gen/wasm_bg.wasm"

await load(bytecode)

export { create }
