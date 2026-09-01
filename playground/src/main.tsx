import { StrictMode } from "react"
import { createRoot } from "react-dom/client"
import * as Sentry from "@sentry/react"
import * as stylex from "@stylexjs/stylex"
import { App } from "./App"
import { colors } from "./tokens.stylex"
import "./index.css"
import "./monacoWorker"
import { init } from "./squawk"

Sentry.init({
  dsn: "https://a974dd404d6ff366b1d62087dd5afaa5@o64108.ingest.us.sentry.io/4509245420994560",
})

const styles = stylex.create({
  fallback: {
    display: "flex",
    alignItems: "center",
    justifyContent: "center",
    height: "100vh",
    fontSize: "3rem",
    lineHeight: 1,
    color: colors.errorText,
  },
  message: {
    display: "flex",
    flexDirection: "column",
  },
  link: {
    textDecoration: "underline",
  },
})

// we want to kick of the wasm load as early as possible, but we still have to
// check that it's loaded later on when we try to call a wasm powered function
init()

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <Sentry.ErrorBoundary
      fallback={() => {
        return (
          <div {...stylex.props(styles.fallback)}>
            <div {...stylex.props(styles.message)}>
              <div>An internal error with Squawk has occured.</div>
              <div>
                Please open an issue at{" "}
                <a
                  href="https://github.com/sbdchd/squawk/issues/new"
                  {...stylex.props(styles.link)}
                >
                  github.com/sbdchd/squawk
                </a>
                !
              </div>
            </div>
          </div>
        )
      }}
    >
      <App />
    </Sentry.ErrorBoundary>
  </StrictMode>,
)
