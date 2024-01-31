// A cloudflare worker script running the hasher to be deployed in miniflare

import * as Hasher from '../src/wasm-import.js'

export default {
  async fetch () {
    const hasher = Hasher.create()
    hasher.write(new Uint8Array([1, 2, 3, 4, 5, 6]))
    // ⚠️ Because digest size will dependen on the payload (padding)
    // we have to determine number of bytes needed after we're done
    // writing payload
    const digest = new Uint8Array(hasher.multihashByteLength())
    hasher.digestInto(digest, 0, true)

    // There's no GC (yet) in WASM so you should free up
    // memory manually once you're done.
    hasher.free()

    return new Response('done')
  }
}
