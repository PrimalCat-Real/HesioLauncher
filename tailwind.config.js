/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./components/**/*.{js,vue,ts}",
    "./layouts/**/*.vue",
    "./pages/**/*.vue",
    'plugins/**/*.{js,ts}',
    'App.{js,ts,vue}',
    'app.{js,ts,vue}',
    'Error.{js,ts,vue}',
    'error.{js,ts,vue}',
    "./nuxt.config.{js,ts}",
  ],
  theme: {
    extend: {
      fontFamily: {
        inter: ["Inter", "sans-serif"],
      },
      colors:{
        dark:{
          default: "#101010",
          200: "#181818",
          400: "#232426",
        },
        light: {
          default: "#EFEFEF",
          200: "#DFDFDF",
          600: "#6A6B6D",
        },
        main: {
          default: "#8774E1"
        }
      },
    },
  },
  plugins: [require("daisyui")],
}

