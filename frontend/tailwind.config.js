/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    '../templates/**/*.html', // Adjust to your Minijinja templates directory
    '../templates/**/*.jinja', // For `.jinja` files if used
  ],
  theme: {
    extend: {
      colors: {
        italianGreen: '#064221ff',
        italianRed: '#901a1eff',
      },
      fontFamily: {
        handwriting: ['"Pacifico"', 'cursive'],
      },
    },
  },
  plugins: [],

};