const got = require('got')

/**
 * Tell if we need the `identity` header.
 *
 * Cloudflare seems to return chunk encoded content if we specify this header
 * for HEAD requests (#75).
 *
 * @param {object} baton
 * @return {boolean}
 */
const needsIdentityHeader = (baton) =>
  !baton.url.includes('unpkg.com')

/**
 * Fetch file to stat from Github.
 *
 * @param {object} baton
 */
async function fetch(baton) {
  try {
    const res = await got[baton.compression ? 'get' : 'head'](baton.url, {
      headers: needsIdentityHeader(baton)
        ? {
          'accept-encoding': 'identity'
        }
        : undefined
    })

    baton.originalSize = Number(res.headers['content-length'])
    baton.size = baton.originalSize
    baton.data = res.body
  }
  catch (err) {
    baton.err = new Error('Unknown path')
    throw baton
  }
}

module.exports = fetch
