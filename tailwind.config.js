/** @type {import('tailwindcss').Config} */
export default {
  content: [
    './src/**/*.{html,ts,svelte}',
  ],
  theme: {
    extend: {
      colors: {
        'dark-indigo': '#2C2C54',
        'deep-purple-gray': '#474787',
        'cool-gray': '#AAABB8',
        'light-gray': '#ECECEC',
      },
    },
  },
  plugins: [],
}

