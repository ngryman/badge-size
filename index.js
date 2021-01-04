const acl = require('./lib/acl')
const color = require('./lib/color')
const compression = require('./lib/compression')
const constraints = require('./lib/constraints')
const fetch = require('./lib/fetch')
const init = require('./lib/init')
const pretty = require('./lib/pretty')
const send = require('./lib/send')

const bind = (fn, ...args) => fn.bind(fn, ...args)

const cond = (fn, param) => async (baton) => {
  if (baton[param] != null) {
    await fn(baton)
  }
}

const tap = (fn) => async (baton) => {
  await fn(baton)
  return baton
}

/**
 * Handle a badge request.
 * It redirects to a shields.io badge of type: `size-{size}-brightgreen`.
 *
 * @param  {ServerRequest} req
 * @param  {ServerResponse} res
 * @return {Promise}
 */
module.exports = function badgeSize(req, res) {
  return init(req)
    .then(tap(acl))
    .then(tap(fetch))
    .then(tap(cond(compression, 'compression')))
    .then(tap(pretty))
    .then(tap(cond(constraints, 'max')))
    .then(tap(color))
    .then(bind(send, res))
    .catch(bind(send, res))
}
