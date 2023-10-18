import * as Hasher from "../src/lib.js"
import * as Link from "multiformats/link"
import { digest, varint } from "multiformats"
import * as Digest from "multiformats/hashes/digest"
import * as Raw from "multiformats/codecs/raw"

const prefix = varint.encodeTo(
  Hasher.code,
  new Uint8Array(varint.encodingLength(Hasher.code))
)

/**
 * @type {import("entail").Suite}
 */
export const testLib = {
  basic: async assert => {
    const hasher = Hasher.create()
    const bytes = new Uint8Array(65).fill(0)
    hasher.write(bytes)

    const digest = new Uint8Array(37)
    hasher.digestInto(digest, 0, true)

    assert.deepEqual(
      digest,
      new Uint8Array([
        ...prefix,
        34, // size
        62, // padding
        2, // height
        55,
        49,
        187,
        153,
        172,
        104,
        159,
        102,
        238,
        245,
        151,
        62,
        74,
        148,
        218,
        24,
        143,
        77,
        220,
        174,
        88,
        7,
        36,
        252,
        111,
        63,
        214,
        13,
        253,
        72,
        131,
        51,
      ])
    )
  },

  test0BytePayload: async assert => {
    const hasher = Hasher.create()
    const output = new Uint8Array(37)

    const end = hasher.digestInto(output, 0, true)
    const digest = Digest.decode(output.subarray(0, end))

    assert.deepEqual(digest.code, Hasher.code)
    assert.deepEqual(digest.size, 34)
    assert.deepEqual(
      digest.bytes,
      new Uint8Array([
        ...prefix,
        34, // size,
        127, // padding
        2, // height
        55,
        49,
        187,
        153,
        172,
        104,
        159,
        102,
        238,
        245,
        151,
        62,
        74,
        148,
        218,
        24,
        143,
        77,
        220,
        174,
        88,
        7,
        36,
        252,
        111,
        63,
        214,
        13,
        253,
        72,
        131,
        51,
      ])
    )
    assert.equal(
      Link.create(Raw.code, digest).toString(),
      "bafkzcibcp4bdomn3tgwgrh3g532zopskstnbrd2n3sxfqbze7rxt7vqn7veigmy"
    )
  },

  test127BytePayload: async assert => {
    const hasher = Hasher.create()
    hasher.write(new Uint8Array(127).fill(0))
    const output = new Uint8Array(64)

    const end = hasher.digestInto(output, 0, true)
    const digest = Digest.decode(output.subarray(0, end))

    assert.deepEqual(digest.code, Hasher.code)
    assert.deepEqual(digest.size, 34)
    assert.deepEqual(
      digest.bytes,
      new Uint8Array([
        ...prefix,
        digest.size, // size,
        0, // padding
        2, // height
        55,
        49,
        187,
        153,
        172,
        104,
        159,
        102,
        238,
        245,
        151,
        62,
        74,
        148,
        218,
        24,
        143,
        77,
        220,
        174,
        88,
        7,
        36,
        252,
        111,
        63,
        214,
        13,
        253,
        72,
        131,
        51,
      ])
    )
    assert.equal(
      Link.create(Raw.code, digest).toString(),
      "bafkzcibcaabdomn3tgwgrh3g532zopskstnbrd2n3sxfqbze7rxt7vqn7veigmy"
    )
  },

  test128BytePayload: async assert => {
    const hasher = Hasher.create()
    hasher.write(new Uint8Array(128).fill(0))
    const output = new Uint8Array(64)

    const end = hasher.digestInto(output, 0, true)
    const digest = Digest.decode(output.subarray(0, end))

    assert.deepEqual(digest.code, Hasher.code)
    assert.deepEqual(digest.size, 34)
    assert.deepEqual(
      digest.bytes,
      new Uint8Array([
        ...prefix,
        digest.size, // size,
        126, // padding
        3, // height
        // root
        100,
        42,
        96,
        126,
        248,
        134,
        176,
        4,
        191,
        44,
        25,
        120,
        70,
        58,
        225,
        212,
        105,
        58,
        192,
        244,
        16,
        235,
        45,
        27,
        122,
        71,
        254,
        32,
        94,
        94,
        117,
        15,
      ])
    )
    assert.equal(
      Link.create(Raw.code, digest).toString(),
      "bafkzcibcpybwiktap34inmaex4wbs6cghlq5i2j2yd2bb2zndn5ep7ralzphkdy"
    )
  },

  testSpec127x4: async assert => {
    const hasher = Hasher.create()
    hasher.write(new Uint8Array(127).fill(0))
    hasher.write(new Uint8Array(127).fill(1))
    hasher.write(new Uint8Array(127).fill(2))
    hasher.write(new Uint8Array(127).fill(3))
    const output = new Uint8Array(64)

    const end = hasher.digestInto(output, 0, true)
    const digest = Digest.decode(output.subarray(0, end))

    assert.deepEqual(digest.code, Hasher.code)
    assert.deepEqual(digest.size, 34)
    assert.deepEqual(
      digest.bytes,
      new Uint8Array([
        ...prefix,
        digest.size, // size,
        0, // padding
        4, // height
        73,
        109,
        174,
        12,
        201,
        226,
        101,
        239,
        229,
        160,
        6,
        232,
        6,
        38,
        165,
        220,
        92,
        64,
        158,
        93,
        49,
        85,
        193,
        57,
        132,
        202,
        246,
        200,
        213,
        207,
        214,
        5,
      ])
    )
    assert.equal(
      Link.create(Raw.code, digest).toString(),
      "bafkzcibcaaces3nobte6ezpp4wqan2age2s5yxcatzotcvobhgcmv5wi2xh5mbi"
    )
  },

  testSpec128x4: async assert => {
    const hasher = Hasher.create()
    hasher.write(new Uint8Array(127).fill(0))
    hasher.write(new Uint8Array(127).fill(1))
    hasher.write(new Uint8Array(127).fill(2))
    hasher.write(new Uint8Array(127).fill(3))
    hasher.write(new Uint8Array(128 * 4 - 127 * 4).fill(0))
    const output = new Uint8Array(64)

    const end = hasher.digestInto(output, 0, true)
    const digest = Digest.decode(output.subarray(0, end))

    assert.deepEqual(digest.code, Hasher.code)
    assert.deepEqual(digest.size, 35)
    assert.deepEqual(
      digest.bytes,
      new Uint8Array([
        ...prefix,
        digest.size, // size,
        248,
        3, // padding
        5, // height
        222,
        104,
        21,
        220,
        179,
        72,
        132,
        50,
        21,
        169,
        77,
        229,
        50,
        149,
        75,
        96,
        190,
        85,
        10,
        75,
        236,
        110,
        116,
        85,
        86,
        101,
        233,
        165,
        236,
        78,
        15,
        60,
      ])
    )
    assert.equal(
      Link.create(Raw.code, digest).toString(),
      "bafkzcibd7abqlxticxolgseegik2stpfgkkuwyf6kufex3doorkvmzpjuxwe4dz4"
    )
  },
  ["testSpec128x4 + 1"]: async assert => {
    const hasher = Hasher.create()
    hasher.write(new Uint8Array(127).fill(0))
    hasher.write(new Uint8Array(127).fill(1))
    hasher.write(new Uint8Array(127).fill(2))
    hasher.write(new Uint8Array(127).fill(3))
    hasher.write(new Uint8Array(128 * 4 - 127 * 4 + 1).fill(0))

    const output = new Uint8Array(64)

    const end = hasher.digestInto(output, 0, true)
    const digest = Digest.decode(output.subarray(0, end))

    assert.deepEqual(digest.code, Hasher.code)
    assert.deepEqual(digest.size, 35)
    assert.deepEqual(
      digest.bytes,
      new Uint8Array([
        ...prefix,
        digest.size, // size,
        247,
        3, // padding
        5, // height
        222,
        104,
        21,
        220,
        179,
        72,
        132,
        50,
        21,
        169,
        77,
        229,
        50,
        149,
        75,
        96,
        190,
        85,
        10,
        75,
        236,
        110,
        116,
        85,
        86,
        101,
        233,
        165,
        236,
        78,
        15,
        60,
      ])
    )
    assert.equal(
      Link.create(Raw.code, digest).toString(),
      "bafkzcibd64bqlxticxolgseegik2stpfgkkuwyf6kufex3doorkvmzpjuxwe4dz4"
    )
  },
}
