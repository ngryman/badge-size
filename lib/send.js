const got = require('got')

/** URLs of shield.io used ot serve badge images. */
const SHIELDS_URL = 'https://img.shields.io/badge'

function getCacheHeaders(duration = 3600) {
  return {
    // Cache both on the CDN and browsers, avoid cache stampede thanks to `stale-while-revalidate`
    "cache-control": `max-age=${duration}, stale-while-revalidate`,
    // set expires to avoid github caching
    //   https://github.com/github/markup/issues/224#issuecomment-48532178
    'expires': new Date(Date.now() + duration * 1000).toUTCString()
  }
}

function getShieldsUrl(baton) {
  const { label, prettySize, color, extension } = baton
  const pathname = `/${label}-${prettySize}-${color}.${extension}`
  const style = baton.style ? `?style=${baton.style}` : ''

  return `${SHIELDS_URL}${pathname}${style}`
}

async function sendBadge(res, baton) {
  const badgeUrl = getShieldsUrl(baton)
  const badgeRes = await got.get(badgeUrl)

  res.writeHead(200, {
    ...getCacheHeaders(),
    'x-shields-url': badgeUrl,
    'content-type': badgeRes.headers['content-type']
  })
  res.end(badgeRes.body)
}

function sendJSON(res, baton) {
  res.writeHead(200, {
    ...getCacheHeaders(),
    'content-type': 'application/json'
  })
  res.end(JSON.stringify({
    prettySize: baton.prettySize,
    originalSize: baton.originalSize,
    size: baton.size,
    color: baton.color
  }))
}

function sendFallbackBadge(res) {
  res.writeHead(200, {
    ...getCacheHeaders(300),
    'content-type': 'image/svg+xml;charset=utf-8'
  })
  res.end(`
    <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="110" height="20" role="img" aria-label="size: critical error">
      <title>size: critical error</title>
      <linearGradient id="s" x2="0" y2="100%"><stop offset="0" stop-color="#bbb" stop-opacity=".1"/>
        <stop offset="1" stop-opacity=".1"/>
      </linearGradient>
      <clipPath id="r">
        <rect width="110" height="20" rx="3" fill="#fff"/>
      </clipPath>
      <g clip-path="url(#r)">
        <rect width="31" height="20" fill="#555"/>
        <rect x="31" width="79" height="20" fill="#9f9f9f"/>
        <rect width="110" height="20" fill="url(#s)"/>
      </g>
      <g fill="#fff" text-anchor="middle" font-family="Verdana,Geneva,DejaVu Sans,sans-serif" text-rendering="geometricPrecision" font-size="110">
        <text aria-hidden="true" x="165" y="150" fill="#010101" fill-opacity=".3" transform="scale(.1)" textLength="210">size</text>
        <text x="165" y="140" transform="scale(.1)" fill="#fff" textLength="210">size</text>
        <text aria-hidden="true" x="695" y="150" fill="#010101" fill-opacity=".3" transform="scale(.1)" textLength="690">critical error</text>
        <text x="695" y="140" transform="scale(.1)" fill="#fff" textLength="690">critical error</text>
      </g>
    </svg>
  `)
}

/**
 * Send the response.
 * For image formats it redirects to shields.io to serve the badge image.
 *
 * @param {ServerResponse} res
 * @param {object} baton
 */
async function send(res, baton) {
  if (baton.err) {
    baton.prettySize = baton.err.message.toLowerCase()
    baton.color = 'lightgrey'
  }

  try {
    if ('json' === baton.extension) {
      sendJSON(res, baton)
    }
    else {
      await sendBadge(res, baton)
    }
  }
  catch (err) {
    sendFallbackBadge(res)
  }
}

module.exports = send
