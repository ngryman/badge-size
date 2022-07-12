const got = require('got')

/**
 * Normalise the URL.
 *
 * Vercel seems to cleanup URLs by removing several ocurrences of / (#86).
 *
 * @param {string} url
 * @return {string}
 */
const normalizeUrl = url => url.replace(/(https?:\/)(\w+)/, '$1/$2')

/**
 * Fetch file to stat from Github.
 *
 * @param {object} baton
 */
async function fetch(baton) {
  const url = normalizeUrl(baton.url)

  try {
    const res = await got(url, {
      headers: {
        'accept-encoding': baton.encoding
      }
    })

    baton.originalSize = res.body.length
    baton.size = baton.originalSize
    baton.data = res.body
  } catch (err) {
    baton.err = new Error('Unknown path')
    throw baton
  }
}

module.exports = fetch
