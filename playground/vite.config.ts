import { defineConfig } from "vite"
import { sentryVitePlugin } from "@sentry/vite-plugin"
import react from "@vitejs/plugin-react"
import tailwindcss from "@tailwindcss/vite"

const ReactCompilerConfig = {}

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    react({
      babel: {
        plugins: [["babel-plugin-react-compiler", ReactCompilerConfig]],
      },
    }),
    tailwindcss(),
    sentryVitePlugin({
      org: "magnus-montis",
      project: "squawk-playground-ui",
    }),
  ],
  build: {
    sourcemap: true,
  },
})
