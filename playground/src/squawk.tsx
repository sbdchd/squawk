import { useEffect, useState } from "react"
import initWasm, { SquawkDatabase } from "./pkg/squawk_wasm"

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

let db: SquawkDatabase | null = null

// We pass in content and version here so that we:
// 1. update the database
// 2. so the react compiler doesn't just cache the functions at their initial
//    results. We need them to be dependent on their input.
//
// We can probably do better than this.
function getDb(content: string, version: number): SquawkDatabase {
  if (db == null) {
    db = new SquawkDatabase()
    db.open_file(content)
  }
  db.update_file(content, version)

  return db
}

function lint(content: string, version: number): Array<LintError> {
  return getDb(content, version).lint()
}

export function inlay_hints(content: string, version: number): InlayHint[] {
  return getDb(content, version).inlay_hints()
}

export function code_actions(
  content: string,
  version: number,
  line: number,
  column: number,
): CodeAction[] | null {
  return getDb(content, version).code_actions(line, column)
}

export function document_symbols(
  content: string,
  version: number,
): DocumentSymbol[] {
  return getDb(content, version).document_symbols()
}

export function hover(
  content: string,
  version: number,
  line: number,
  column: number,
): string | null {
  return getDb(content, version).hover(line, column)
}

export function goto_definition(
  content: string,
  version: number,
  line: number,
  column: number,
): Array<LocationRange> {
  return getDb(content, version).goto_definition(line, column)
}

export function find_references(
  content: string,
  version: number,
  line: number,
  column: number,
): LocationRange[] {
  return getDb(content, version).find_references(line, column)
}

export function selection_ranges(
  content: string,
  version: number,
  positions: Array<{ line: number; column: number }>,
): SelectionRange[][] {
  return getDb(content, version).selection_ranges(positions)
}

export function folding_ranges(
  content: string,
  version: number,
): FoldingRange[] {
  return getDb(content, version).folding_ranges()
}

export function completion(
  content: string,
  version: number,
  line: number,
  column: number,
): CompletionItem[] {
  return getDb(content, version).completion(line, column)
}

export function dump_cst(content: string, version: number): string {
  return getDb(content, version).dump_cst()
}

export function dump_tokens(content: string, version: number): string {
  return getDb(content, version).dump_tokens()
}

export function useErrors(text: string, version: number) {
  const isReady = useWasmStatus()
  return isReady ? lint(text, version) : []
}

export function useDumpCst(text: string, version: number): string {
  const isReady = useWasmStatus()
  return isReady ? dump_cst(text, version) : ""
}

export function useDumpTokens(text: string, version: number): string {
  const isReady = useWasmStatus()
  return isReady ? dump_tokens(text, version) : ""
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
  file: "current" | "builtins"
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

export interface FoldingRange {
  start_line: number
  end_line: number
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
