// noinspection JSUnresolvedVariable

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["../../crates/libs/lib-web/src/templates/*.rs"],
  theme: {
    extend: {},
  },
  plugins: [require("@tailwindcss/typography"), require("daisyui")],
}
