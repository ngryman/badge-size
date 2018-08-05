/** URLs of shield.io used ot serve badge images. */
const SHIELDS_URL = 'https://img.shields.io/badge'

function sendRedirect(res, baton) {
  const pathname = encodeURI(`/${baton.label}-${baton.value}-${baton.color}.${baton.extension}`)
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

function sendJSON(res, baton) {
  res.writeHead(200, {
    'content-type': 'application/json'
  })
  res.end(JSON.stringify({
    size: baton.value,
    color: baton.color
  }))
}

/**
 * Send the response.
 * For image formats it redirects to shields.io to serve the badge image.
 *
 * @param {ServerResponse} res
 * @param {object} baton
 */
function send(res, baton) {
  if (baton.err) {
    baton.value = baton.err.message.toLowerCase()
    baton.color = 'lightgrey'
  }

  if ('json' === baton.extension) {
    sendJSON(res, baton)
  }
  else {
    sendRedirect(res, baton)
  }
}

module.exports = send
