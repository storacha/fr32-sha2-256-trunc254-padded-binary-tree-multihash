# fr32-sha2-256-trunc254-padded-binary-tree-multihash

Rust implementation of V2 Piece Multihash per [FIP0069][] compiled to WASM and wrapped in JS package.

## Usage

```typescript
import Hasher from "fr32-sha2-256-trunc254-padded-binary-tree-multihash"


export const digestStream = async (source: AsyncIterable<Uint8Array>) => {
  // allocate buffer to hold the multihash
  const digest = new Uint8Array(Hasher.MAX_SIZE)
  
  const hasher = Hasher.create()
  for (const chunk of source) {
    hasher.write(chunk)
  }

  // Write digest and capture end offset
  const offset = hasher.digestInto(
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

  // Return subarray containing multihash bytes
  return digest.subarray(0, offset)
}
```

You could also know precise number of bytes that will be needed to hold the multihash once you're done feeding bytes into it

```ts
import Hasher from "fr32-sha2-256-trunc254-padded-binary-tree-multihash"


export const digestStream2 = async (source: AsyncIterable<Uint8Array>) => {
  const hasher = Hasher.create()
  for (const chunk of source) {
    hasher.write(chunk)
  }

  // allocate buffer to hold the multihash
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

ℹ️ You can recycle hasher to use on a different payload by calling `hasher.reset()`

[FIP0069]:https://github.com/filecoin-project/FIPs/blob/master/FRCs/frc-0069.md
