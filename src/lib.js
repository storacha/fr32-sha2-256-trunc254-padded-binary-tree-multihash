// @ts-expect-error no type declarations
import bytecode from '../gen/wasm_bg.wasm'

import load, { create } from "../gen/wasm.js"
import { code, CODE_LENGTH, HEIGHT_SIZE, ROOT_SIZE, MAX_SIZE } from './constant.js'
export * from "./type.js"
export { code, CODE_LENGTH, HEIGHT_SIZE, ROOT_SIZE, MAX_SIZE }

await load(bytecode)

export { create }
