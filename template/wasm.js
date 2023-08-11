// import bytecode from "./bytecode.js"

// export const activate = () => init(bytecode)

export { __wbg_init }

/**
 * @param {string|URL} url
 */
const fetch = async url => {
  if (url.toString().startsWith("file:")) {
    const fs = await import("node:fs/promises")
    const data = await fs.readFile(url)
    return data
  } else {
    return globalThis.fetch(url)
  }
}
