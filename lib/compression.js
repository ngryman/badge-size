const brotliSize = require('brotli-size')
const gzipSize = require('gzip-size')

/**
 * Stat compressed size of the file if requested.
 *
 * @param {object} baton
 */
async function compression(baton) {
  switch (baton.compression) {
    case 'gzip':
      baton.size = await gzipSize(baton.data)
      break

    case 'brotli':
      baton.size = await brotliSize(baton.data)
      break

    default:
      baton.err = new Error('Unknown compression')
      throw baton
  }
}

module.exports = compression
