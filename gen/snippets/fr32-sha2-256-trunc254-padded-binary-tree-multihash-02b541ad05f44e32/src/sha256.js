/**
 *
 * @param {Uint8Array} payload
 * @param {Uint8Array} dst
 */
export async function sha256_into(payload, dst) {
  const hash = await crypto.subtle.digest("SHA-256", payload)
  dst.set(new Uint8Array(hash))
}
