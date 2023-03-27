/**
 * Update color based on size constraints.
 *
 * @param {object} baton
 */
function constraints(baton) {
  baton.color =
      baton.size > baton.max
        ? 'red'
        : baton.size > baton.softmax
          ? 'yellow'
          : baton.color;
}

module.exports = constraints
