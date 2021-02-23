/** @type {import("snowpack").SnowpackUserConfig } */
module.exports = {
  mount: {
    'src': { url: '/build' },
    'public': { url: '/', static: true },
  },
  plugins: [
    '@snowpack/plugin-svelte',
    '@snowpack/plugin-typescript',
  ],
  packageOptions: {
    /* ... */
  },
  devOptions: {
    port: 4000,
    open: 'none',
    output: 'stream', // disable clearing of terminal
  },
  buildOptions: {
    out: './public/build',
    sourcemap: true,
    clean: true,
  },
  optimize: {
    entrypoints: ['src/main.js'],
    bundle: true,
    target: 'es2017',
  },
}
