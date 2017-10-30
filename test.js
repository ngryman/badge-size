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

test('accept brotli compression', async t => {
  const res = await request(t, '/baxterthehacker/public-repo/master/README.md.svg?compression=brotli')
  assert(t, res, '/brotli size-18 B-brightgreen.svg')
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
  assert(t, res, '/size-unknown compression-lightgrey.svg')
})

test('Size is valid', async t => {
  const res = await request(t, '/baxterthehacker/public-repo/master/README.md.svg?max=100000&softmax=200000')
  assert(t, res, '/size-14 B-brightgreen.svg')
})

test('Single param is used if softmax param is omitted', async t => {
  const res2 = await request(t, '/baxterthehacker/public-repo/master/README.md.svg?max=1')
  assert(t, res2, '/size-14 B-red.svg')
})

test('Size checking is bypassed if max is not provided', async t => {
  const res3 = await request(t, '/baxterthehacker/public-repo/master/README.md.svg?softmax=1')
  assert(t, res3, '/size-14 B-brightgreen.svg')
})

test('Color is red if size greater than softmax threshold', async t => {
  const res = await request(t, '/baxterthehacker/public-repo/master/README.md.svg?max=1&softmax=1')
  assert(t, res, '/size-14 B-red.svg')
})

test('Color is yellow if size greater than max but less than softmax', async t => {
  const res = await request(t, '/baxterthehacker/public-repo/master/README.md.svg?max=1&softmax=100000')
  assert(t, res, '/size-14 B-yellow.svg')
})

test('max and softmax params override color option ', async t => {
  const res = await request(t, '/baxterthehacker/public-repo/master/README.md.svg?max=1&softmax=2&color=orange')
  assert(t, res, '/size-14 B-red.svg')
})
