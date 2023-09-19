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
 * The digest for the multihash is 33 bytes. The first byte defines the height
 * of the tree and the remaining 32 bytes are the sha-256 digest of the root
 * node.
 *
 * @type {33}
 */
export const size = 33

/**
 * Multihash prefix encoding the multihash code and digest size.
 */
export const prefix = new Uint8Array([145, 32, 33])

export { create }
