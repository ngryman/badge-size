import test from 'ava'
import injectThen from 'inject-then'
import server from './'

const SHIELDS_URL = 'https://img.shields.io/badge'

test.before.cb(t => {
  server.register(injectThen, t.end)
})

test.after(() => server.stop())

test('redirect to shields.io', async t => {
  const res = await server.injectThen({
    method: 'GET',
    url: '/baxterthehacker/public-repo/master/README.md.svg'
  })

  t.is(res.statusCode, 302)
  t.is(res.headers.location, `${SHIELDS_URL}/size-14 B-brightgreen.svg`)
})

test('accept gzip compression', async t => {
  const res = await server.injectThen({
    method: 'GET',
    url: '/baxterthehacker/public-repo/master/README.md.svg?compression=gzip'
  })

  t.is(res.statusCode, 302)
  t.is(res.headers.location, `${SHIELDS_URL}/gzip size-34 B-brightgreen.svg`)
})

test('accept other branch names', async t => {
  const res = await server.injectThen({
    method: 'GET',
    url: '/baxterthehacker/public-repo/changes/README.md.svg'
  })

  t.is(res.statusCode, 302)
  t.is(res.headers.location, `${SHIELDS_URL}/size-12 B-brightgreen.svg`)
})

test('allow other image extensions', async t => {
  const res = await server.injectThen({
    method: 'GET',
    url: '/baxterthehacker/public-repo/changes/README.md.png'
  })

  t.is(res.statusCode, 302)
  t.is(res.headers.location, `${SHIELDS_URL}/size-12 B-brightgreen.png`)
})

test('default to svg if no comprehensible extension is found', async t => {
  const res = await server.injectThen({
    method: 'GET',
    url: '/baxterthehacker/public-repo/changes/README.md'
  })

  t.is(res.statusCode, 302)
  t.is(res.headers.location, `${SHIELDS_URL}/size-12 B-brightgreen.svg`)
})

test('accept a custom label', async t => {
  const res = await server.injectThen({
    method: 'GET',
    url: '/baxterthehacker/public-repo/master/README.md?label=taille'
  })

  t.is(res.statusCode, 302)
  t.is(res.headers.location, `${SHIELDS_URL}/taille-14 B-brightgreen.svg`)
})

test('accept a custom color', async t => {
  const res = await server.injectThen({
    method: 'GET',
    url: '/baxterthehacker/public-repo/master/README.md?color=bada55'
  })

  t.is(res.statusCode, 302)
  t.is(res.headers.location, `${SHIELDS_URL}/size-14 B-bada55.svg`)
})

test('accept a custom style', async t => {
  const res = await server.injectThen({
    method: 'GET',
    url: '/baxterthehacker/public-repo/master/README.md?style=flat'
  })

  t.is(res.statusCode, 302)
  t.is(res.headers.location, `${SHIELDS_URL}/size-14 B-brightgreen.svg?style=flat`)
})

test('reject empty paths', async t => {
  const res = await server.injectThen({
    method: 'GET',
    url: '/'
  })

  t.is(res.statusCode, 302)
  t.is(res.headers.location, `${SHIELDS_URL}/size-empty path-lightgrey.svg`)
})

test('reject invalid path', async t => {
  const res = await server.injectThen({
    method: 'GET',
    url: '/non-sense/query'
  })

  t.is(res.statusCode, 302)
  t.is(res.headers.location, `${SHIELDS_URL}/size-unknown path-lightgrey.svg`)
})

test('reject other types of compression', async t => {
  const res = await server.injectThen({
    method: 'GET',
    url: '/baxterthehacker/public-repo/master/README.md.svg?compression=lzma'
  })

  t.is(res.statusCode, 302)
  t.is(res.headers.location, `${SHIELDS_URL}/gzip size-unknown compression-lightgrey.svg`)
})
