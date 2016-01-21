/*eslint-env mocha */
/*eslint-disable padded-blocks, no-unused-expressions */

import server from './'
import { expect } from 'chai'
import injectThen from 'inject-then'

const SHIELDS_URL = 'https://img.shields.io/badge'

describe('http://img.badgesize.io', () => {

  before((done) => {
    server.register(injectThen, done)
  })

  after(() => server.stop())

  it('redirects to shields.io', () => {
    return server.injectThen({
      method: 'GET',
      url: '/baxterthehacker/public-repo/master/README.md.svg'
    }).then((res) => {
      expect(res.statusCode).to.equal(200)
      expect(res.headers['x-uri']).to.equal(`${SHIELDS_URL}/size-14 B-brightgreen.svg`)
    })
  })

  it('accepts gzip compression', () => {
    return server.injectThen({
      method: 'GET',
      url: '/baxterthehacker/public-repo/master/README.md.svg?compression=gzip'
    }).then((res) => {
      expect(res.statusCode).to.equal(200)
      expect(res.headers['x-uri']).to.equal(`${SHIELDS_URL}/gzip size-34 B-brightgreen.svg`)
    })
  })

  it('accepts other branch names', () => {
    return server.injectThen({
      method: 'GET',
      url: '/baxterthehacker/public-repo/changes/README.md.svg'
    }).then((res) => {
      expect(res.statusCode).to.equal(200)
      expect(res.headers['x-uri']).to.equal(`${SHIELDS_URL}/size-12 B-brightgreen.svg`)
    })
  })

  it('allows other image extensions', () => {
    return server.injectThen({
      method: 'GET',
      url: '/baxterthehacker/public-repo/changes/README.md.png'
    }).then((res) => {
      expect(res.statusCode).to.equal(200)
      expect(res.headers['x-uri']).to.equal(`${SHIELDS_URL}/size-12 B-brightgreen.png`)
    })
  })

  it('defaults to svg if no comprehensible extension is found', () => {
    return server.injectThen({
      method: 'GET',
      url: '/baxterthehacker/public-repo/changes/README.md'
    }).then((res) => {
      expect(res.statusCode).to.equal(200)
      expect(res.headers['x-uri']).to.equal(`${SHIELDS_URL}/size-12 B-brightgreen.svg`)
    })
  })

  it('accepts a custom label', () => {
    return server.injectThen({
      method: 'GET',
      url: '/baxterthehacker/public-repo/master/README.md?label=taille'
    }).then((res) => {
      expect(res.statusCode).to.equal(200)
      expect(res.headers['x-uri']).to.equal(`${SHIELDS_URL}/taille-14 B-brightgreen.svg`)
    })
  })

  it('accepts a custom color', function() {
    return server.injectThen({
      method: 'GET',
      url: '/baxterthehacker/public-repo/master/README.md?color=bada55'
    }).then((res) => {
      expect(res.statusCode).to.equal(200)
      expect(res.headers['x-uri']).to.equal(`${SHIELDS_URL}/size-14 B-bada55.svg`)
    })
  })

  it('accepts a custom style', function() {
    return server.injectThen({
      method: 'GET',
      url: '/baxterthehacker/public-repo/master/README.md?style=flat'
    }).then((res) => {
      expect(res.statusCode).to.equal(200)
      expect(res.headers['x-uri']).to.equal(`${SHIELDS_URL}/size-14 B-brightgreen.svg?style=flat`)
    })
  })

  context('with invalid arguments', () => {

    it('sets size to undefined', () => {
      return server.injectThen({
        method: 'GET',
        url: '/non-sense/query'
      }).then((res) => {
        expect(res.statusCode).to.equal(200)
        expect(res.headers['x-uri']).to.equal(`${SHIELDS_URL}/size-unknown-brightgreen.svg`)
      })
    })

  })

})
