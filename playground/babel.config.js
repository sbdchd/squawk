import styleXPlugin from "@stylexjs/babel-plugin"

export default {
  presets: ["@babel/preset-typescript"],
  plugins: [
    [
      styleXPlugin,
      {
        runtimeInjection: false,
        unstable_moduleResolution: {
          type: "commonJS",
          rootDir: import.meta.dirname,
        },
      },
    ],
  ],
}
