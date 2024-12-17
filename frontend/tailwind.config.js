/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    '../templates/**/*.html', // Adjust to your Minijinja templates directory
    '../templates/**/*.jinja', // For `.jinja` files if used
  ],
  theme: {
    extend: {},
  },
  plugins: [],

};
