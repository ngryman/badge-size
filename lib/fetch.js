const got = require('got')

/**
 * Fetch file to stat from Github.
 *
 * @param {object} baton
 */
async function fetch(baton) {
  try {
    const res = await got[baton.compression ? 'get' : 'head'](baton.url, {
      headers: {
        'accept-encoding': 'identity'
      }
    })

    baton.size = Number(res.headers['content-length'])
    baton.data = res.body
  }
  catch (err) {
    baton.err = new Error('Unknown path')
    throw baton
  }
}

module.exports = fetch
