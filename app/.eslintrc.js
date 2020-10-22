module.exports = {
  env: {
    es2020: true,
    node: true,
    browser: true,
  },
  extends: 'eslint:recommended',
  parserOptions: {
    ecmaVersion: 2020,
    sourceType: 'module',
  },
  plugins: [
    'svelte3',
  ],
  ignorePatterns: ['**/node_modules'],
  overrides: [
    {
      files: ['src/*.svelte'],
      processor: 'svelte3/svelte3',
    },
  ],
  rules: {
    'comma-dangle': [ 'error', 'always-multiline' ],
    'linebreak-style': [ 'error', 'unix' ],
    indent: [ 'error', 2 ],
    quotes: [ 'error', 'single' ],
    semi: [ 'error', 'never' ],
    'no-unused-vars': [
      'error',
      { args: 'none' },
    ],
  },
}
