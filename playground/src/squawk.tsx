import { useEffect, useState } from "react"
import initWasm, {
  dump_cst,
  dump_tokens,
  lint as lint_,
  goto_definition as goto_definition_,
  hover as hover_,
  find_references as find_references_,
  document_symbols as document_symbols_,
  code_actions as code_actions_,
  inlay_hints as inlay_hints_,
  selection_ranges as selection_ranges_,
  completion as completion_,
} from "./pkg/squawk_wasm"

export type TextEdit = {
  start_line_number: number
  start_column: number
  end_line_number: number
  end_column: number
  text: string
}

export type Fix = {
  title: string
  edits: TextEdit[]
}

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
  fix?: Fix
}

function lint(text: string): Array<LintError> {
  return lint_(text)
}

export function inlay_hints(content: string): InlayHint[] {
  return inlay_hints_(content)
}

export function code_actions(
  content: string,
  line: number,
  column: number,
): CodeAction[] | null {
  return code_actions_(content, line, column)
}

export function document_symbols(content: string): DocumentSymbol[] {
  return document_symbols_(content)
}

export function hover(
  content: string,
  line: number,
  column: number,
): string | null {
  return hover_(content, line, column)
}

export function goto_definition(
  content: string,
  line: number,
  column: number,
): Array<LocationRange> {
  return goto_definition_(content, line, column)
}

export function find_references(
  content: string,
  line: number,
  column: number,
): LocationRange[] {
  return find_references_(content, line, column)
}

export function selection_ranges(
  content: string,
  positions: Array<{ line: number; column: number }>,
): SelectionRange[][] {
  return selection_ranges_(content, positions)
}

export function completion(
  content: string,
  line: number,
  column: number,
): CompletionItem[] {
  return completion_(content, line, column)
}

export function useErrors(text: string) {
  const isReady = useWasmStatus()
  return isReady ? lint(text) : []
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

interface LocationRange {
  start_line: number
  start_column: number
  end_line: number
  end_column: number
}

interface CodeAction {
  title: string
  edits: TextEdit[]
  kind: string
}

export interface DocumentSymbol {
  name: string
  detail: string | null
  kind: string
  start_line: number
  start_column: number
  end_line: number
  end_column: number
  selection_start_line: number
  selection_start_column: number
  selection_end_line: number
  selection_end_column: number
  children: DocumentSymbol[]
}

interface InlayHint {
  line: number
  column: number
  label: string
  kind: string
}

export interface SelectionRange {
  start_line: number
  start_column: number
  end_line: number
  end_column: number
}

export interface CompletionItem {
  label: string
  kind: string
  detail: string | null
  insert_text: string | null
  insert_text_format: string | null
  trigger_completion_after_insert: boolean
}
