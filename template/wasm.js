import bytecode from "./wasm_bg.js"

export { __wbg_init }

/**
 * @param {string|URL} url
 */
const fetch = async url => {
  // // if (url.toString().startsWith("file:")) {
  // //   const fs = await import("node:fs/promises")
  // //   const data = await fs.readFile(url)
  // //   return data
  // // } else {
  // //   return globalThis.fetch(url)
  // // }
  // // const buffer = await new Blob([atob(bytecode)]).arrayBuffer()

  // // console.log(buffer)
  // // return new Uint8Array(buffer)
  // return bytecode

  return globalThis.fetch(`data:application/wasm;base64,${bytecode}`)
}
