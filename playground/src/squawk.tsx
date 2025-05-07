import { useEffect, useState } from "react"
import initWasm, {
  dump_cst,
  dump_tokens,
  lint as lint_,
} from "./pkg/squawk_wasm"

export type LintError = {
  code: string
  message: string
  severity: "Hint" | "Info" | "Warning" | "Error"
  start_line_number: number
  start_column: number
  end_line_number: number
  end_column: number
  range_start: number
  range_end: number
  messages: string[]
}

function lintWithTypes(text: string): Array<LintError> {
  return lint_(text)
}

export function useErrors(text: string) {
  const isReady = useWasmStatus()
  return isReady ? lintWithTypes(text) : []
}

export function useDumpCst(text: string): string {
  const isReady = useWasmStatus()
  return isReady ? dump_cst(text) : ""
}

export function useDumpTokens(text: string): string {
  const isReady = useWasmStatus()
  return isReady ? dump_tokens(text) : ""
}

let isStartingAlready: { promise: Promise<unknown>; start: number } | null =
  null

export function init() {
  isStartingAlready = { promise: initWasm(), start: performance.now() }
}

function useWasmStatus() {
  const [isReady, setIsReady] = useState(false)
  useEffect(() => {
    if (isStartingAlready != null) {
      isStartingAlready.promise.then(() => {
        setIsReady(true)
      })
      return
    }
    const start = performance.now()
    const promise = initWasm()
      .then(() => {
        setIsReady(true)
        const end = performance.now()
        console.log(`wasm setup done in ${end - start} ms`)
      })
      .catch(() => {
        console.error("problem initializing wasm")
        setIsReady(true)
      })
      .finally(() => {})
    isStartingAlready = { promise, start: start }
  }, [])
  return isReady
}
