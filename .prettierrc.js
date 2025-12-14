const SETUP_OUTPUTS = process.env.DECANCER_SETUP_OUTPUTS
  ? JSON.parse(process.env.DECANCER_SETUP_OUTPUTS)
  : {}
const plugins = []

if (SETUP_OUTPUTS.java_affected === 'true') {
  plugins.push(require.resolve('prettier-plugin-java'))
}

module.exports = {
  semi: false,
  singleQuote: true,
  trailingComma: 'none',
  arrowParens: 'avoid',
  htmlWhitespaceSensitivity: 'ignore',
  plugins
}
