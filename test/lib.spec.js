import * as Hasher from "../src/lib.js"

/**
 * @type {import("entail").Suite}
 */
export const testLib = {
  basic: async assert => {
    const hasher = Hasher.create()
    const bytes = new Uint8Array(65).fill(0)
    hasher.write(bytes)

    const digest = new Uint8Array(36)
    hasher.digestInto(digest, 0, true)

    assert.deepEqual(
      digest,
      new Uint8Array([
        145, 32, 33, 2, 55, 49, 187, 153, 172, 104, 159, 102, 238, 245, 151, 62,
        74, 148, 218, 24, 143, 77, 220, 174, 88, 7, 36, 252, 111, 63, 214, 13,
        253, 72, 131, 51,
      ])
    )
  },
}
