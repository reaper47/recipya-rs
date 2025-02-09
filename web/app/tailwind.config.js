// noinspection JSUnresolvedVariable

/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["../../src/server/templates/*.rs"],
    daisyui: {
        darkTheme: "forest",
        themes: true,
    },
    theme: {
        extend: {},
    },
    plugins: [require("@tailwindcss/typography"), require("daisyui")],
}
