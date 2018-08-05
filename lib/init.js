const url = require('url')

/** URLs of GitHub content. */
const GITHUB_URL = 'https://raw.githubusercontent.com'

/**
 * Parse infos from request parameters and build a baton.
 *
 * @param  {ServerRequest} req
 * @return {object}
 */
async function init(req) {
  const { pathname, query } = url.parse(req.url, true)

  const baton = {
    label: query.label || `${query.compression ? `${query.compression} ` : ''}size`,
    color: query.color || 'brightgreen',
    style: query.style || null,
    max: query.max || null,
    softmax: query.softmax || Infinity,
    value: 'unknown',
    extension: 'svg',
    size: 0,
    compression: query.compression || null,
    compressedSize: 0,
    err: null
  }

  // empty path
  if ('/' === pathname) {
    baton.err = new Error('Empty path')
    throw baton
  }

  // url or github path
  if (pathname.startsWith('/http')) {
    baton.url = pathname.substr(1)
  }
  else {
    baton.url = `${GITHUB_URL}${pathname}`
  }

  // image or json extension
  const index = pathname.lastIndexOf('.')
  if (-1 !== index) {
    baton.extension = pathname.substr(index + 1)
    if (-1 === 'svg|png|jpg|json'.indexOf(baton.extension)) {
      baton.extension = 'svg'
    }
    else {
      baton.url = `${GITHUB_URL}${pathname.substr(0, index)}`
    }
  }

  return baton
}

module.exports = init
