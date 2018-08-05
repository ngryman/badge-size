const prettyBytes = require('pretty-bytes')

/**
 * Make file size pretty to read.
 *
 * @param {object} baton
 */
async function pretty(baton) {
  baton.value = prettyBytes(baton.compressedSize || baton.size)
}

module.exports = pretty
