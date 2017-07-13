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

function assert(t, res, pathname) {
  t.is(res.statusCode, 303)
  t.is(res.headers.location, `${SHIELDS_URL}${encodeURI(pathname)}`)
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
  assert(t, res, '/size-14 B-brightgreen.svg')
})

test('accept any url', async t => {
  const res = await request(t,
    '/https://raw.githubusercontent.com/baxterthehacker/public-repo/master/README.md.svg')
  assert(t, res, '/size-14 B-brightgreen.svg')
})

test('accept gzip compression', async t => {
  const res = await request(t, '/baxterthehacker/public-repo/master/README.md.svg?compression=gzip')
  assert(t, res, '/gzip size-34 B-brightgreen.svg')
})

test('accept other branch names', async t => {
  const res = await request(t, '/baxterthehacker/public-repo/changes/README.md.svg')
  assert(t, res, '/size-12 B-brightgreen.svg')
})

test('allow other image extensions', async t => {
  const res = await request(t, '/baxterthehacker/public-repo/changes/README.md.png')
  assert(t, res, '/size-12 B-brightgreen.png')
})

test('default to svg if no comprehensible extension is found', async t => {
  const res = await request(t, '/baxterthehacker/public-repo/changes/README.md')
  assert(t, res, '/size-12 B-brightgreen.svg')
})

test('accept a custom label', async t => {
  const res = await request(t, '/baxterthehacker/public-repo/master/README.md?label=taille')
  assert(t, res, '/taille-14 B-brightgreen.svg')
})

test('accept a custom color', async t => {
  const res = await request(t, '/baxterthehacker/public-repo/master/README.md?color=bada55')
  assert(t, res, '/size-14 B-bada55.svg')
})

test('accept a custom style', async t => {
  const res = await request(t, '/baxterthehacker/public-repo/master/README.md?style=flat')
  assert(t, res, '/size-14 B-brightgreen.svg?style=flat')
})

test('reject empty paths', async t => {
  const res = await request(t, '/')
  assert(t, res, '/size-empty path-lightgrey.svg')
})

test('reject invalid path', async t => {
  const res = await request(t, '/non-sense/query')
  assert(t, res, '/size-unknown path-lightgrey.svg')
})

test('reject other types of compression', async t => {
  const res = await request(t, '/baxterthehacker/public-repo/master/README.md.svg?compression=lzma')
  assert(t, res, '/gzip size-unknown compression-lightgrey.svg')
})

test('Size is valid', async t => {
  const res = await request(t, '/baxterthehacker/public-repo/master/README.md.svg?pass=100000&fail=200000')
  assert(t, res, '/size-14 B-brightgreen.svg')
})

test('Single param is used if fail param is omitted', async t => {
  const res2 = await request(t, '/baxterthehacker/public-repo/master/README.md.svg?pass=1')
  assert(t, res2, '/size-14 B-red.svg')
})

test('Single param is used if pass param is omitted', async t => {
  const res3 = await request(t, '/baxterthehacker/public-repo/master/README.md.svg?fail=1')
  assert(t, res3, '/size-14 B-red.svg')
})

test('Color is red if size greater than fail threshold', async t => {
  const res = await request(t, '/baxterthehacker/public-repo/master/README.md.svg?pass=1&fail=1')
  assert(t, res, '/size-14 B-red.svg')
})

test('Color is yellow if size greater than pass threshold but less than fail threshold', async t => {
  const res = await request(t, '/baxterthehacker/public-repo/master/README.md.svg?pass=1&fail=10000')
  assert(t, res, '/size-14 B-yellow.svg')
})
