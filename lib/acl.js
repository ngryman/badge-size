/** List of denied user agents */
const UA_DENYLIST = [
  'Baiduspider'
].join('|')

/**
 * Throw if the user agent is part of the deny list.
 *
 * @param {object} baton
 */
function acl(baton) {
  if (~baton.userAgent.indexOf(UA_DENYLIST)) {
    baton.err = new Error('Access denied')
    throw baton
  }
}

module.exports = acl
