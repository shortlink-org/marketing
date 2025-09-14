const shortlink = require('@shortlink-org/eslint-plugin-shortlink')

module.exports = [
  ...shortlink,
  {
    ignores: ['node_modules', 'out', '.*', 'public', 'test'],
  },
]
