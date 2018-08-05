/*eslint max-len: 0*/
import test from 'ava'
import micro from 'micro'
import got from 'got'
import listen from 'test-listen'
import badgeSize from './'

const SHIELDS_URL = 'https://img.shields.io/badge'

function request(t, path) {
  return got(`${t.context.url}${path}`, {
    followRedirect: false
  })
}

function assertHeaders(t, res, pathname) {
  t.is(res.statusCode, 303)
  t.is(res.headers.location, `${SHIELDS_URL}${encodeURI(pathname)}`)
}

function assertBody(t, res, body) {
  t.is(res.body, JSON.stringify(body))
}

test.beforeEach(async t => {
  t.context.server = micro(badgeSize)
  t.context.url = await listen(t.context.server)
})

test.afterEach.cb(t => {
  t.context.server.close(t.end)
})

test('redirect to shields.io', async t => {
  const res = await request(t, '/baxterthehacker/public-repo/master/README.md.svg')
  assertHeaders(t, res, '/size-14 B-brightgreen.svg')
})

test('accept any url', async t => {
  const res = await request(t,
    '/https://raw.githubusercontent.com/baxterthehacker/public-repo/master/README.md.svg')
  assertHeaders(t, res, '/size-14 B-brightgreen.svg')
})

test('accept gzip compression', async t => {
  const res = await request(t, '/baxterthehacker/public-repo/master/README.md.svg?compression=gzip')
  assertHeaders(t, res, '/gzip size-34 B-brightgreen.svg')
})

test('accept brotli compression', async t => {
  const res = await request(t, '/baxterthehacker/public-repo/master/README.md.svg?compression=brotli')
  assertHeaders(t, res, '/brotli size-18 B-brightgreen.svg')
})

test('accept other branch names', async t => {
  const res = await request(t, '/baxterthehacker/public-repo/changes/README.md.svg')
  assertHeaders(t, res, '/size-12 B-brightgreen.svg')
})

test('allow other image extensions', async t => {
  const res = await request(t, '/baxterthehacker/public-repo/changes/README.md.png')
  assertHeaders(t, res, '/size-12 B-brightgreen.png')
})

test('default to svg if no comprehensible extension is found', async t => {
  const res = await request(t, '/baxterthehacker/public-repo/changes/README.md')
  assertHeaders(t, res, '/size-12 B-brightgreen.svg')
})

test('accept a custom label', async t => {
  const res = await request(t, '/baxterthehacker/public-repo/master/README.md?label=taille')
  assertHeaders(t, res, '/taille-14 B-brightgreen.svg')
})

test('accept a custom color', async t => {
  const res = await request(t, '/baxterthehacker/public-repo/master/README.md?color=bada55')
  assertHeaders(t, res, '/size-14 B-bada55.svg')
})

test('accept a custom style', async t => {
  const res = await request(t, '/baxterthehacker/public-repo/master/README.md?style=flat')
  assertHeaders(t, res, '/size-14 B-brightgreen.svg?style=flat')
})

test('reject empty paths', async t => {
  const res = await request(t, '/')
  assertHeaders(t, res, '/size-empty path-lightgrey.svg')
})

test('reject invalid path', async t => {
  const res = await request(t, '/non-sense/query')
  assertHeaders(t, res, '/size-unknown path-lightgrey.svg')
})

test('reject other types of compression', async t => {
  const res = await request(t, '/baxterthehacker/public-repo/master/README.md.svg?compression=lzma')
  assertHeaders(t, res, '/lzma size-unknown compression-lightgrey.svg')
})

test('check size and set color to green when size is less than `max`', async t => {
  const res = await request(t, '/baxterthehacker/public-repo/master/README.md.svg?max=100')
  assertHeaders(t, res, '/size-14 B-brightgreen.svg')
})

test('check size and set color to green when size is less than `softmax`', async t => {
  const res = await request(t, '/baxterthehacker/public-repo/master/README.md.svg?max=100&softmax=100')
  assertHeaders(t, res, '/size-14 B-brightgreen.svg')
})

test('check size and set color to yellow when size is less than `max` but more than `softmax`', async t => {
  const res = await request(t, '/baxterthehacker/public-repo/master/README.md.svg?max=100&softmax=13')
  assertHeaders(t, res, '/size-14 B-yellow.svg')
})

test('check size do not override a custom color if size is less than `max`', async t => {
  const res = await request(t, '/baxterthehacker/public-repo/master/README.md.svg?max=100&color=orange')
  assertHeaders(t, res, '/size-14 B-orange.svg')
})

test('check size do override a custom color if size is more than `max`', async t => {
  const res = await request(t, '/baxterthehacker/public-repo/master/README.md.svg?max=13&color=orange')
  assertHeaders(t, res, '/size-14 B-red.svg')
})

test('ignore `softmax` if `max` is not present', async t => {
  const res = await request(t, '/baxterthehacker/public-repo/master/README.md.svg?softmax=13')
  assertHeaders(t, res, '/size-14 B-brightgreen.svg')
})

test('accept json format', async t => {
  const res = await request(t, '/baxterthehacker/public-repo/master/README.md.json')
  assertBody(t, res, { prettySize: '14 B', originalSize: 14, size: 14, color: 'brightgreen' })
})

test('accept json format and differenciate original size from compressed size', async t => {
  const res = await request(t, '/baxterthehacker/public-repo/master/README.md.json?compression=gzip')
  assertBody(t, res, { prettySize: '34 B', originalSize: 14, size: 34, color: 'brightgreen' })
})
