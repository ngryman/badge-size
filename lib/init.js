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
    originalSize: 0,
    size: 0,
    compression: query.compression || null,
    userAgent: req.headers['user-agent'],
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
  const index = baton.url.lastIndexOf('.')
  if (-1 !== index) {
    baton.extension = baton.url.substr(index + 1)
    if (!/svg|png|json$/.test(baton.extension)) {
      baton.extension = 'svg'
    }
    else {
      baton.url = baton.url.substr(0, index)
    }
  }

  return baton
}

module.exports = init
