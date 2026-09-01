export default {
  plugins: {
    "@stylexjs/postcss-plugin": {
      include: ["src/**/*.{ts,tsx}"],
    },
    autoprefixer: {},
  },
}
