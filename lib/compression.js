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
      baton.compressedSize = await gzipSize(baton.data)
      break

    case 'brotli':
      baton.compressedSize = await brotliSize(baton.data)
      break

    default:
      baton.err = new Error('Unknown compression')
      throw baton
  }
}

module.exports = compression
