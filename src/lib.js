import load, { create } from "../gen/wasm.js"
import { code, CODE_LENGTH, HEIGHT_SIZE, ROOT_SIZE, MAX_SIZE } from './constant.js'
export * from "./type.js"
export { code, CODE_LENGTH, HEIGHT_SIZE, ROOT_SIZE, MAX_SIZE }

await load()

export { create }
