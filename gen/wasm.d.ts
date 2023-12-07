/* tslint:disable */
/* eslint-disable */
/**
* @returns {PieceHasher}
*/
export function create(): PieceHasher;
/**
*/
export class PieceHasher {
  free(): void;
/**
* Creates a new hasher
*/
  constructor();
/**
* @returns {bigint}
*/
  count(): bigint;
/**
* Resets the hasher state
*/
  reset(): void;
/**
* @param {Uint8Array} bytes
*/
  write(bytes: Uint8Array): void;
/**
* @param {Uint8Array} target
* @param {number | undefined} offset
* @param {boolean | undefined} use_prefix
* @returns {number}
*/
  digestInto(target: Uint8Array, offset?: number, use_prefix?: boolean): number;
/**
* @returns {number}
*/
  digestByteLength(): number;
/**
* @returns {number}
*/
  multihashByteLength(): number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_piecehasher_free: (a: number) => void;
  readonly piecehasher_count: (a: number) => number;
  readonly piecehasher_reset: (a: number) => void;
  readonly piecehasher_write: (a: number, b: number, c: number, d: number) => void;
  readonly piecehasher_digestInto: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => number;
  readonly piecehasher_digestByteLength: (a: number) => number;
  readonly piecehasher_multihashByteLength: (a: number) => number;
  readonly create: () => number;
  readonly piecehasher_create: () => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
