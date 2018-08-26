/** List of color aliases */
const COLOR_ALIASES = {
  brightgreen: '44cc11',
  green: '97ca00',
  yellowgreen: 'a4a61d',
  yellow: 'dfb317',
  orange: 'fe7d37',
  red: 'e05d44',
  lightgrey: '9f9f9f',
  blue: '007ec6'
}

/**
 * Get hexadecimal code from aliased colors
 *
 * @param {object} baton
 */
function color(baton) {
  baton.color = COLOR_ALIASES[baton.color] || baton.color
}

module.exports = color
