/** List of denied user agents */
const UA_DENYLIST = ['Baiduspider'].join('|')

/** List of denied URLs */
const URL_DENYLIST = ['vxe-table'].join('|')

/**
 * Throw if the user agent is part of the deny list.
 *
 * @param {object} baton
 */
function acl(baton) {
  const { req } = baton

  if (~req.headers['user-agent'].indexOf(UA_DENYLIST) || ~req.url.indexOf(URL_DENYLIST)) {
    baton.err = new Error('unavailable')
    throw baton
  }
}

module.exports = acl
