const url = require('url')
const got = require('got')
const gzipSize = require('gzip-size')
const prettyBytes = require('pretty-bytes')

/** URLs of services in use. */
const GITHUB_URL = 'https://raw.githubusercontent.com'
const SHIELDS_URL = 'https://img.shields.io/badge'

/**
 * Parse infos from request parameters and build a baton.
 *
 * @param  {ServerRequest} req
 * @return {Promise}
 */
function parse(req) {
  return new Promise((resolve, reject) => {
    const { pathname, query } = url.parse(req.url, true)

    let baton = {
      label: query.label || ((query.compression ? 'gzip ' : '') + 'size'),
      color: query.color || 'brightgreen',
      style: query.style || null,
      max: query.max || null,
      softmax: query.softmax || null,
      value: 'unknown',
      extension: 'svg',
      size: 0,
      compression: query.compression,
      compressedSize: 0,
      err: null
    }

    // empty path
    if ('/' === pathname) {
      baton.err = new Error('Empty path')
      return reject(baton)
    }

    // url or github path
    if (pathname.startsWith('/http')) {
      baton.url = pathname.substr(1)
    }
    else {
      baton.url = `${GITHUB_URL}${pathname}`
    }

    // image extension
    let index = pathname.lastIndexOf('.')
    if (-1 !== index) {
      baton.extension = pathname.substr(index + 1)
      if (-1 === 'svg|png|jpg'.indexOf(baton.extension)) {
        baton.extension = 'svg'
      }
      else {
        baton.url = `${GITHUB_URL}${pathname.substr(0, index)}`
      }
    }

    resolve(baton)
  })
}

/**
 * Fetch file to stat from Github.
 *
 * @param  {object} baton
 * @return {Promise}
 */
function fetch(baton) {
  return new Promise((resolve, reject) => {
    got[baton.compression ? 'get' : 'head'](baton.url, {
      headers: {
        'accept-encoding': 'identity'
      }
    }).then(res => {
      baton.size = Number(res.headers['content-length'])
      baton.data = res.body
      resolve(baton)
    }).catch(err => {
      baton.err = 'Unknown path'
      return reject(baton)
    })
  })
}

/**
 * Stat compressed size of the file if requested.
 *
 * @param  {object} baton
 * @return {object|Promise}
 */
function compressed(baton) {
  if (!baton.compression) return baton

  return new Promise((resolve, reject) => {
    baton.compressedSize = baton.size

    if ('gzip' === baton.compression) {
      gzipSize(baton.data, (err, size) => {
        /* istanbul ignore if  */
        if (err) {
          baton.err = err
          return reject(baton)
        }

        baton.compressedSize = size
        resolve(baton)
      })
    }
    else {
      baton.err = 'Unknown compression'
      reject(baton)
    }
  })
}

/**
 * Make file size pretty to read.
 *
 * @param  {object} baton
 * @return {object}
 */
function pretty(baton) {
  baton.value = prettyBytes(baton.compressedSize || baton.size)
  return baton
}

/**
 * Set color based on size.
 *
 * @param  {object} baton
 * @return {object}
 */
function updateColor(baton) {
  if (!baton.max) return baton

  if (baton.size > baton.max) {
    baton.color = 'red'
    if (baton.softmax && baton.size < baton.softmax) {
      baton.color = 'yellow'
    }
  }

  return baton
}

/**
 * Redirect to shields.io to serve the badge image.
 *
 * @param  {ServerResponse} res
 * @return {function}
 */
function redirect(res) {
  return function(baton) {
    if (baton.err) {
      baton.value = ('string' === typeof baton.err ?
        baton.err :
        baton.err.message
      ).toLowerCase()
      baton.color = 'lightgrey'
    }

    let pathname = encodeURI(
      `/${baton.label}-${baton.value}-${baton.color}.${baton.extension}`
    )
    let badgeUrl = `${SHIELDS_URL}${pathname}`
    if (baton.style) badgeUrl += `?style=${baton.style}`

    res.writeHead(303, {
      'location': badgeUrl,
      // align on github raw cdn which caches content for 5 minutes
      'cache-control': 'max-age=300',
      // set expires to avoid github caching
      //   https://github.com/github/markup/issues/224#issuecomment-48532178
      'expires': new Date(Date.now() + 300 * 1000).toUTCString()
    })
    res.end()
  }
}

/* -------------------------------------------------------------------------- */

/**
 * Handle a badge request.
 * It redirects to a shields.io badge of type: `size-{size}-brightgreen`.
 *
 * @param  {ServerRequest} req
 * @param  {ServerResponse} res
 * @return {Promise}
 */
module.exports = function badgeSize(req, res) {
  return parse(req)
    .then(fetch)
    .then(compressed)
    .then(pretty)
    .then(updateColor)
    .then(redirect(res))
    .catch(redirect(res))
}
