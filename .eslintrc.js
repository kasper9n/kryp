module.exports = {
  env: {
    es2020: true,
    node: true,
    browser: true,
  },
  extends: ['eslint:recommended'],
  parser: '@typescript-eslint/parser',
  parserOptions: {
    ecmaVersion: 2020,
    sourceType: 'module',
  },
  plugins: [
    'svelte3',
    '@typescript-eslint',
  ],
  ignorePatterns: ['**/node_modules'],
  settings: {
    'svelte3/ignore-styles': () => true,
    'svelte3/typescript': require('typescript'),
  },
  overrides: [
    {
      files: ['*.svelte'],
      processor: 'svelte3/svelte3',
    },
  ],
  rules: {
    indent: ['error', 2],
    quotes: ['error', 'single', { 'avoidEscape': true }],
    'comma-dangle': ['error', 'always-multiline'],
    semi: ['error', 'never'],
    'no-unused-vars': 0, // disabled because of bug https://github.com/sveltejs/eslint-plugin-svelte3/pull/65
    // 'no-unused-vars': [
    //   'error',
    //   { args: 'none' },
    // ],
    'linebreak-style': ['error', 'unix'],
    'no-extra-parens': ['error', 'all'],
    'no-loss-of-precision': ['error'],
    'space-before-function-paren': ['error', 'never'],
    'object-curly-spacing': ['error', 'always'],
    'array-bracket-spacing': ['error', 'never', {}],
  },
}
