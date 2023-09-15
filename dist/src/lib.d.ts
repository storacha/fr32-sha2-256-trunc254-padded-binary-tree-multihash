export * from "./type.js";
/**
 * @see https://github.com/multiformats/multicodec/pull/331/files
 */
export const name: "fr32-sha2-256-trunc254-padded-binary-tree";
/**
 * @type {API.MulticodecCode<0x1011, typeof name>}
 * @see https://github.com/multiformats/multicodec/pull/331/files
 */
export const code: API.MulticodecCode<0x1011, typeof name>;
/**
 * The digest for the multihash is 33 bytes. The first byte defines the height
 * of the tree and the remaining 32 bytes are the sha-256 digest of the root
 * node.
 *
 * @type {33}
 */
export const size: 33;
export { create };
import * as API from "./type.js";
import { create } from "../gen/wasm.js";
//# sourceMappingURL=lib.d.ts.map