# fr32-sha2-256-trunc254-padded-binary-tree-multihash

Rust implementation of V2 Piece Multihash per [FIP0069][] compiled to WASM and wrapped in JS package.

## Usage

```typescript
import Hasher from "fr32-sha2-256-trunc254-padded-binary-tree-multihash"


export const digestStream = async (source: AsyncIterable<Uint8Array>) => {
  const hasher = Hasher.create()
  for await (const chunk of source) {
    hasher.write(chunk)
  }

  // Allocate buffer to hold the multihash
  // ⚠️ Calling hasher.write may affect bytes required for digest
  // If you need to pre-allocate see next example
  const digest = new Uint8Array(hasher.multihashByteLength())
  // Write digest and capture end offset
  hasher.digestInto(
    // into provided buffer
    digest,
    // at 0 byte offset
    0,
    // and include multihash prefix
    true
  )

  // There's no GC (yet) in WASM so you should free up
  // memory manually once you're done.
  hasher.free()


  return digest
}
```

Please note that multihash size is not fixed, so if you need to slab allocate
it is best to assume `MAX_SIZE` for each digest.

```typescript
import Hasher from "fr32-sha2-256-trunc254-padded-binary-tree-multihash"


export const concatDigest = async (left: AsyncIterable<Uint8Array>, right: AsyncIterable<Uint8Array>) => {
  // allocate buffer to hold two multihashes
  // ℹ️ We may not utilize full capacity but allocating more is better than resizing
  const buffer = new Uint8Array(2 * Hasher.MAX_SIZE)
  
  const hasher = Hasher.create()
  for await (const chunk of left) {
    hasher.write(chunk)
  }

  // Write digest and capture end offset
  const offset = hasher.digestInto(
    // into provided buffer
    buffer,
    // at 0 byte offset
    0,
    // and include multihash prefix
    true
  )

  // Now we need to reset the hasher to start digesting second stream
  hasher.reset()
  for await (const chunk of right) {
    hasher.write(chunk)
  }

  // Write second digest from the last offset
  const end = hasher.digestInto(
    // into provided buffer
    buffer,
    // at 0 byte offset
    offset,
    // and include multihash prefix
    true
  )

  // There's no GC (yet) in WASM so you should free up
  // memory manually once you're done.
  hasher.free()

  // Return subarray trimming unutilized bytes
  return digest.subarray(0, end)
}
```

### Environments that do not support top level await

The main module in this library uses a top-level await to load `wasm`. In environments that
do not support top-level await (ie, legacy browser environments and many bundlers that build
for them) you can use the `async` module like this:

```javascript
import { digest } from "fr32-sha2-256-trunc254-padded-binary-tree-multihash/async"

export const createDigest = async (bytes: Uint8Array) => {
  return await digest(bytes)
}
```



[FIP0069]:https://github.com/filecoin-project/FIPs/blob/master/FRCs/frc-0069.md
