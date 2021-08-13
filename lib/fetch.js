const got = require('got')

/**
 * Limit the transfer size in bytes.
 *
 * For security reasons (#16).
 *
 * @param {object} baton
 * @return {boolean}
 */
const DOWNLOAD_LIMIT = 50000

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
    const download = got.get(url)
    download.on('downloadProgress', ({ transferred, total, percent }) => {
      if (Math.max(transferred, total) > DOWNLOAD_LIMIT && percent !== 1) {
        download.cancel('exceeded')
      }
    })
    const res = await download

    baton.originalSize = Buffer.byteLength(res.body)
    baton.size = baton.originalSize
    baton.data = res.body
  }
  catch (err) {
    baton.err = 'CancelError' === err.name ? err : new Error('Unknown path')
    throw baton
  }
}

module.exports = fetch
