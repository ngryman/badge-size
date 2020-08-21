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
 * Normalise the URL.
 *
 * Vercel seems to cleanup URLs by removing several ocurrences of / (#86).
 *
 * @param {string} url
 * @return {string}
 */
const normalizeUrl = (url) =>
  url.replace(/(https?:\/)(\w+)/, '$1/$2')

/**
 * Fetch file to stat from Github.
 *
 * @param {object} baton
 */
async function fetch(baton) {
  const url = normalizeUrl(baton.url)

  try {
    const res = await got[baton.compression ? 'get' : 'head'](url, {
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
