// noinspection JSUnresolvedVariable

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["../../src/web/templates/*.rs"],
  theme: {
    extend: {},
  },
  plugins: [require("@tailwindcss/typography"), require("daisyui")],
}
