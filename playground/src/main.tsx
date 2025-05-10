import { StrictMode } from "react"
import { createRoot } from "react-dom/client"
import * as Sentry from "@sentry/react"
import { App } from "./App"
import "./index.css"
import "./monacoWorker"
import { init } from "./squawk"

Sentry.init({
  dsn: "https://a974dd404d6ff366b1d62087dd5afaa5@o64108.ingest.us.sentry.io/4509245420994560",
})

// we want to kick of the wasm load as early as possible, but we still have to
// check that it's loaded later on when we try to call a wasm powered function
init()

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <Sentry.ErrorBoundary
      fallback={() => {
        return (
          <div className="flex items-center justify-center h-screen text-5xl text-orange-700 ">
            <div className="flex flex-col">
              <div>An internal error with Squawk has occured.</div>
              <div>
                Please open an issue at{" "}
                <a
                  href="https://github.com/sbdchd/squawk/issues/new"
                  className="underline"
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
