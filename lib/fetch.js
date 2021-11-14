const got = require('got')

/** Limit the transfer size in bytes. */
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
    download.on('downloadProgress', ({ transferred, total = 0, percent }) => {
      if (Math.max(transferred, total) > DOWNLOAD_LIMIT && percent !== 1) {
        download.cancel()
      }
    })
    const res = await download

    baton.originalSize = res.rawBody.byteLength
    baton.size = baton.originalSize
    baton.data = res.rawBody
  }
  catch (err) {
    baton.err = new Error('CancelError' === err.name ? 'Exceeded' : 'Unknown path')
    throw baton
  }
}

module.exports = fetch
