const colors = require('tailwindcss/colors')

colors.cyan['550'] = 'hsl(189, 94%, 39%)'

const config = {
  content: ['./src/**/*.{html,js,svelte,ts}'],

  theme: {
    extend: {
      colors: {
        primary: colors.red,
        secondary: colors.gray,
        complementary: colors.blue,
        neutral: colors.neutral,
      },
      transitionTimingFunction: {
        md: 'cubic-bezier(0.4, 0.0, 0.2, 1)',
      },
    },
  },

  plugins: [require('@tailwindcss/forms')],
}

module.exports = config
