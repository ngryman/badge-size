import got from 'got'
import gzipSize from 'gzip-size'
import Hapi from 'hapi'
import prettyBytes from 'pretty-bytes'

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
  return new Promise(function(resolve, reject) {
    let baton = {
      label: req.query.label || ((req.query.compression ? 'gzip ' : '') + 'size'),
      color: req.query.color || 'brightgreen',
      style: req.query.style || null,
      value: 'unknown',
      extension: 'svg',
      size: 0,
      compression: req.query.compression,
      compressedSize: 0
    }

    // empty path
    if (!req.params.path) return reject(baton)

    // url and image extension
    baton.url = `${GITHUB_URL}/${req.params.path || ''}`
    let index = req.params.path.lastIndexOf('.')
    if (-1 !== index) {
      baton.extension = req.params.path.substr(index + 1)
      if (-1 === 'svg|png|jpg'.indexOf(baton.extension)) {
        baton.extension = 'svg'
      }
      else {
        baton.url = `${GITHUB_URL}/${req.params.path.substr(0, index)}`
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
  return new Promise(function(resolve, reject) {
    got[baton.compression ? 'get' : 'head'](baton.url, {
      headers: {
        'accept-encoding': 'identity'
      }
    }, function(err, data, res) {
      if (err) return reject(baton)
      baton.size = Number(res.headers['content-length'])
      baton.data = data
      resolve(baton)
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

  return new Promise(function(resolve, reject) {
    baton.compressedSize = baton.size

    if ('gzip' === baton.compression) {
      gzipSize(baton.data, function(err, size) {
        if (err) return reject(baton)
        baton.compressedSize = size
        resolve(baton)
      })
    }
    else {
      resolve(baton)
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
 * Proxy response from shields.io to serve the badge image.
 *
 * @param  {Reply} reply
 * @return {function}
 */
function proxy(reply) {
  return function(baton) {
    let badgeUrl = `${SHIELDS_URL}/${baton.label}-${baton.value}-${baton.color}.${baton.extension}`
    if (baton.style) badgeUrl += `?style=${baton.style}`

    reply.proxy({
      uri: badgeUrl,
      passThrough: true,
      onResponse: function(error, res, request, reply, settings, ttl) {
        reply(res).header('X-Uri', badgeUrl)
      }
    })
  }
}

/* -------------------------------------------------------------------------- */

/**
 * Handle a badge request.
 * It redirects to a shields.io badge of type: `size-{size}-brightgreen`.
 *
 * @param  {ServerRequest} req
 * @param  {Reply} reply
 */
function badgeHandler(req, reply) {
  parse(req)
  .then(fetch)
  .then(compressed)
  .then(pretty)
  .then(proxy(reply))
  .catch(proxy(reply))
}

/** Configure & start the server. */
let server = new Hapi.Server()
server.connection({ port: process.env.PORT || 3000 })
server.route({
  method: 'GET',
  path: '/{path*}',
  handler: badgeHandler
})
server.start()

/** Convenience export for tests. */
export default server
