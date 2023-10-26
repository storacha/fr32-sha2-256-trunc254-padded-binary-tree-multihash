import load, { create } from "../gen/wasm.js"
import * as API from "./type.js"
export * from "./type.js"

await load()

/**
 * @see https://github.com/multiformats/multicodec/pull/331/files
 */
export const name = /** @type {const} */ (
  "fr32-sha2-256-trunc254-padded-binary-tree"
)

/**
 * @type {API.MulticodecCode<0x1011, typeof name>}
 * @see https://github.com/multiformats/multicodec/pull/331/files
 */
export const code = 0x1011

/**
 * Multihash code size in varint bytes
 */
export const CODE_LENGTH = 2

/**
 * Max padding size in bytes
 */
const MAX_PADDING_SIZE = 9

/**
 * One byte is used to store the tree height.
 */
export const HEIGHT_SIZE = 1

/**
 * Amount of bytes used to store the tree root.
 */
export const ROOT_SIZE = 32

/**
 * Size of the multihash digest in bytes.
 */
const MAX_DIGEST_SIZE = MAX_PADDING_SIZE + HEIGHT_SIZE + ROOT_SIZE

/**
 * Multihash digest length in varint bytes
 */
const MAX_DIGEST_LENGTH = 1

/**
 * Max number of bytes required to fit this multihash
 */
export const MAX_SIZE = CODE_LENGTH + MAX_DIGEST_LENGTH + MAX_DIGEST_SIZE

export { create }
