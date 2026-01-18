import * as monaco from "monaco-editor"
import {
  code_actions,
  completion,
  document_symbols,
  find_references,
  goto_definition,
  hover,
  inlay_hints,
  selection_ranges,
  DocumentSymbol,
} from "./squawk"

export async function provideInlayHints(
  model: monaco.editor.ITextModel,
): Promise<monaco.languages.InlayHintList> {
  const content = model.getValue()
  if (!content) return { hints: [], dispose: () => {} }

  try {
    const wasmHints = inlay_hints(content)

    const hints = wasmHints.map((hint) => ({
      label: hint.label,
      position: {
        lineNumber: hint.line + 1,
        column: hint.column + 1,
      },
      kind:
        hint.kind === "type"
          ? monaco.languages.InlayHintKind.Type
          : monaco.languages.InlayHintKind.Parameter,
    }))

    return { hints, dispose: () => {} }
  } catch (e) {
    console.error("Error in provideInlayHints:", e)
    return { hints: [], dispose: () => {} }
  }
}

export async function provideDocumentSymbols(
  model: monaco.editor.ITextModel,
): Promise<monaco.languages.DocumentSymbol[]> {
  const content = model.getValue()
  if (!content) return []

  try {
    const symbols = document_symbols(content)

    return symbols.map((symbol) => convertDocumentSymbol(symbol))
  } catch (e) {
    console.error("Error in provideDocumentSymbols:", e)
    return []
  }
}

function convertDocumentSymbol(
  symbol: DocumentSymbol,
): monaco.languages.DocumentSymbol {
  return {
    name: symbol.name,
    detail: symbol.detail || "",
    kind: convertSymbolKind(symbol.kind),
    range: {
      startLineNumber: symbol.start_line + 1,
      startColumn: symbol.start_column + 1,
      endLineNumber: symbol.end_line + 1,
      endColumn: symbol.end_column + 1,
    },
    selectionRange: {
      startLineNumber: symbol.selection_start_line + 1,
      startColumn: symbol.selection_start_column + 1,
      endLineNumber: symbol.selection_end_line + 1,
      endColumn: symbol.selection_end_column + 1,
    },
    children: symbol.children.map((child) => convertDocumentSymbol(child)),
    tags: [],
  }
}

function convertSymbolKind(kind: string): monaco.languages.SymbolKind {
  switch (kind) {
    case "table":
      return monaco.languages.SymbolKind.Class
    case "function":
      return monaco.languages.SymbolKind.Function
    case "column":
      return monaco.languages.SymbolKind.Property
    default:
      return monaco.languages.SymbolKind.Variable
  }
}

export async function provideCodeActions(
  model: monaco.editor.ITextModel,
  position: monaco.Position,
): Promise<monaco.languages.CodeAction[]> {
  const content = model.getValue()
  if (!content) return []

  try {
    const actions = code_actions(
      content,
      position.lineNumber - 1,
      position.column - 1,
    )

    if (!actions) return []

    return actions.map((action) => ({
      title: action.title,
      kind: action.kind,
      edit: {
        edits: action.edits.map((edit) => ({
          resource: model.uri,
          versionId: model.getVersionId(),
          textEdit: {
            range: {
              startLineNumber: edit.start_line_number + 1,
              startColumn: edit.start_column + 1,
              endLineNumber: edit.end_line_number + 1,
              endColumn: edit.end_column + 1,
            },
            text: edit.text,
          },
        })),
      },
    }))
  } catch (e) {
    console.error("Error in provideCodeActions:", e)
    return []
  }
}

export async function provideHover(
  model: monaco.editor.ITextModel,
  position: monaco.Position,
): Promise<monaco.languages.Hover | null> {
  const content = model.getValue()
  if (!content) return null

  try {
    const result = hover(content, position.lineNumber - 1, position.column - 1)

    if (!result) return null

    return {
      contents: [{ value: result }],
    }
  } catch (e) {
    console.error("Error in provideHover:", e)
    return null
  }
}

export async function provideDefinition(
  model: monaco.editor.ITextModel,
  position: monaco.Position,
): Promise<Array<monaco.languages.Location> | null> {
  const content = model.getValue()
  if (!content) return null

  try {
    const results = goto_definition(
      content,
      position.lineNumber - 1,
      position.column - 1,
    )

    if (!results) return null

    return results.map((results) => ({
      uri: model.uri,
      range: {
        startLineNumber: results.start_line + 1,
        startColumn: results.start_column + 1,
        endLineNumber: results.end_line + 1,
        endColumn: results.end_column + 1,
      },
    }))
  } catch (e) {
    console.error("Error in provideDefinition:", e)
    return null
  }
}

export async function provideReferences(
  model: monaco.editor.ITextModel,
  position: monaco.Position,
): Promise<monaco.languages.Location[]> {
  const content = model.getValue()
  if (!content) return []

  try {
    const results = find_references(
      content,
      position.lineNumber - 1,
      position.column - 1,
    )

    return results.map((result) => ({
      uri: model.uri,
      range: {
        startLineNumber: result.start_line + 1,
        startColumn: result.start_column + 1,
        endLineNumber: result.end_line + 1,
        endColumn: result.end_column + 1,
      },
    }))
  } catch (e) {
    console.error("Error in provideReferences:", e)
    return []
  }
}

export async function provideSelectionRanges(
  model: monaco.editor.ITextModel,
  positions: monaco.Position[],
): Promise<monaco.languages.SelectionRange[][]> {
  const content = model.getValue()
  if (!content) return []

  try {
    const wasmPositions = positions.map((pos) => ({
      line: pos.lineNumber - 1,
      column: pos.column - 1,
    }))

    const results = selection_ranges(content, wasmPositions)

    return results.map((ranges) =>
      ranges.map((range) => ({
        range: {
          startLineNumber: range.start_line + 1,
          startColumn: range.start_column + 1,
          endLineNumber: range.end_line + 1,
          endColumn: range.end_column + 1,
        },
      })),
    )
  } catch (e) {
    console.error("Error in provideSelectionRanges:", e)
    return []
  }
}

function convertCompletionKind(
  kind: string,
): monaco.languages.CompletionItemKind {
  switch (kind) {
    case "keyword":
      return monaco.languages.CompletionItemKind.Keyword
    case "table":
      return monaco.languages.CompletionItemKind.Class
    case "column":
      return monaco.languages.CompletionItemKind.Field
    case "function":
      return monaco.languages.CompletionItemKind.Function
    case "schema":
      return monaco.languages.CompletionItemKind.Module
    case "type":
      return monaco.languages.CompletionItemKind.TypeParameter
    case "snippet":
      return monaco.languages.CompletionItemKind.Snippet
    default:
      return monaco.languages.CompletionItemKind.Text
  }
}

export async function provideCompletionItems(
  model: monaco.editor.ITextModel,
  position: monaco.Position,
): Promise<monaco.languages.CompletionList> {
  const content = model.getValue()
  if (!content) return { suggestions: [] }

  try {
    const items = completion(
      content,
      position.lineNumber - 1,
      position.column - 1,
    )

    const suggestions: monaco.languages.CompletionItem[] = items.map(
      (item) => ({
        label: item.label,
        kind: convertCompletionKind(item.kind),
        detail: item.detail ?? undefined,
        insertText: item.insert_text ?? item.label,
        insertTextRules:
          item.insert_text_format === "snippet"
            ? monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet
            : undefined,
        command: item.trigger_completion_after_insert
          ? { id: "editor.action.triggerSuggest", title: "Trigger Suggest" }
          : undefined,
        sortText: item.kind === "schema" ? `z${item.label}` : item.label,
        range: undefined as unknown as monaco.IRange,
      }),
    )

    return { suggestions }
  } catch (e) {
    console.error("Error in provideCompletionItems:", e)
    return { suggestions: [] }
  }
}
