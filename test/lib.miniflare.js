import { Miniflare } from 'miniflare'
import path from 'path'

/**
 * @type {import("entail").Suite}
 */
export const testLib = {
  basic: async assert => {
    // A Basic test loading the cloudflare module into a worker
    const miniflare = new Miniflare({
      modules: true,
      port: 8788,
      modulesRules: [
        { type: "ESModule", include: ["**/*.js"], fallthrough: true },
        { type: "CompiledWasm", include: ["**/*.wasm"] },
      ],
      scriptPath: path.join(process.cwd(), '/test/cloudflare-worker.js') // Path to your Cloudflare Worker script
    })

    // Make a request to the running Worker
    const response = await miniflare.dispatchFetch('http://localhost:8787')

    assert.equal(response.status, 200)
    await miniflare.dispose()
  }
}
