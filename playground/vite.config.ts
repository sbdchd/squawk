import { defineConfig } from "vite"
import { sentryVitePlugin } from "@sentry/vite-plugin"
import react from "@vitejs/plugin-react"

const ReactCompilerConfig = { panicThreshold: "all_errors" }

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    react({
      babel: {
        // babel.config.js has the stylex plugin
        configFile: true,
        plugins: [["babel-plugin-react-compiler", ReactCompilerConfig]],
      },
    }),
    sentryVitePlugin({
      org: "magnus-montis",
      project: "squawk-playground-ui",
    }),
  ],
  build: {
    sourcemap: true,
  },
})
