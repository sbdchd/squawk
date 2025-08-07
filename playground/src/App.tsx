import { useState, useEffect, useLayoutEffect, useRef } from "react"
import * as monaco from "monaco-editor"
import { LintError, Fix, useDumpCst, useDumpTokens, useErrors } from "./squawk"
import {
  compress,
  compressToEncodedURIComponent,
  decompress,
  decompressFromEncodedURIComponent,
} from "lz-string"

const modes = ["Lint", "Syntax Tree", "Tokens"] as const
const STORAGE_KEY = "playground-history-v1"

type Mode = (typeof modes)[number]

const DEFAULT_CONTENT = `\
create table users (
  -- squawk-ignore prefer-bigint-over-int
  id serial
);

-- oops we forgot this
alter table users 
  -- squawk-ignore prefer-robust-stmts
  add column is_admin boolean default func();

select * from users;
`

const SETTINGS = {
  value: DEFAULT_CONTENT,
  language: "pgsql",
  tabSize: 2,
  theme: "vs-dark",
  minimap: { enabled: false },
  automaticLayout: true,
  scrollBeyondLastLine: false,
  folding: false,
  showFoldingControls: "never",
  occurrencesHighlight: "off",
  stickyScroll: { enabled: false },
  fontSize: 16,
  // otherwise it looks bad on mobile
  fontFamily:
    '-apple-system-ui-monospace, "SF Mono", ui-monospace, "Cascadia Code", Menlo, Monaco, "Segoe UI Mono", Consolas, monospace',
  wordWrap: "on",
  renderWhitespace: "boundary",
  guides: { indentation: false },
  lineNumbersMinChars: 3,
} satisfies monaco.editor.IStandaloneEditorConstructionOptions

function clx(...args: (string | undefined | number | false)[]): string {
  const classes = new Set<string>()
  for (const arg of args) {
    if (!arg) {
      continue
    } else if (typeof arg === "string" || typeof arg === "number") {
      classes.add(String(arg))
    } else {
      assertNever(arg)
    }
  }
  return [...classes].join(" ")
}

function initialMode(): Mode | null {
  const mode = localStorage.getItem("play-mode-v1")
  if (modes.includes(mode as Mode)) {
    return mode as Mode
  }
  if (mode == "none") {
    return null
  }
  return "Lint"
}

function useMode() {
  const [mode, setActiveMode] = useState<Mode | null>(() => initialMode())

  useEffect(() => {
    try {
      localStorage.setItem("play-mode-v1", mode ?? "none")
    } catch {
      // pass
    }
  }, [mode])

  return [mode, setActiveMode] as const
}

function initialValue(): string | null {
  // for example:
  // http://localhost:5173/#code/M4UwNiDGAuAECMBuIA
  const uriData = window.location.hash.split("code/")[1] as string | undefined
  if (uriData) {
    return decompressFromEncodedURIComponent(uriData)
  }
  const history = localStorage.getItem(STORAGE_KEY)
  if (history != null) {
    return decompress(history)
  }
  return null
}

export function App() {
  const [mode, setActiveMode] = useMode()
  const [text, setState] = useState(() => initialValue() ?? SETTINGS.value)

  const markers = useMarkers(text)

  return (
    <div className="flex flex-col h-screen">
      <Nav>
        <a
          href="https://squawkhq.com"
          target="_blank"
          className="px-3 py-1 rounded hover:bg-gray-700 transition-colors"
        >
          Docs
        </a>
        <a
          href="https://squawkhq.com/docs/rules"
          target="_blank"
          className="px-3 py-1 rounded hover:bg-gray-700 transition-colors"
        >
          Rules
        </a>
        <a
          href="https://github.com/sbdchd/squawk"
          target="_blank"
          className="px-3 py-1 rounded hover:bg-gray-700 transition-colors"
        >
          GitHub
        </a>
        <ShareButton text={text} />
      </Nav>
      <div className="flex flex-1 mt-1">
        <div
          className={clx(
            "grid grid-cols-1 flex-1 overflow-auto",
            mode != null && "md:grid-cols-2",
          )}
        >
          <Editor
            onChange={(text) => {
              setState(text)
            }}
            autoFocus
            markers={markers}
            settings={{ ...SETTINGS, value: text }}
            onSave={handleSave}
          />
          {mode === "Syntax Tree" ? (
            <SyntaxTreePanel text={text} />
          ) : mode === "Tokens" ? (
            <TokenPanel text={text} />
          ) : mode === "Lint" ? (
            <ErrorPanel errors={markers} />
          ) : mode == null ? null : (
            assertNever(mode)
          )}
        </div>
        <Controls activeMode={mode} onModeChange={setActiveMode} />
      </div>
    </div>
  )
}

function TokenPanel({ text }: { text: string }) {
  const value = useDumpTokens(text)
  return (
    <Editor
      value={value}
      settings={{
        ...SETTINGS,
        fontSize: 14,
        value,
        language: "rast",
        readOnly: true,
        lineNumbers: "off",
      }}
    />
  )
}

function Controls({
  activeMode,
  onModeChange,
}: {
  activeMode: Mode | null
  onModeChange: (mode: Mode | null) => void
}) {
  return (
    <div className="bg-[rgb(30,30,30)] border-l border-[#282828] px-4 py-2">
      <div className="flex flex-col gap-1">
        {modes.map((mode) => (
          <button
            key={mode}
            onClick={() => {
              onModeChange(activeMode === mode ? null : mode)
            }}
            className={clx(
              "w-full px-2 py-1 text-sm rounded transition-colors",
              activeMode === mode
                ? "bg-blue-600 text-white"
                : "text-gray-300 hover:bg-gray-700",
            )}
          >
            {mode}
          </button>
        ))}
      </div>
    </div>
  )
}

function assertNever(x: never): never {
  throw new Error(`expected never, got ${x}`)
}

function Editor({
  onChange,
  autoFocus,
  settings,
  value,
  markers,
  onSave,
}: {
  value?: string
  autoFocus?: boolean
  onChange?: (_: string) => void
  onSave?: (_: string) => void
  settings: monaco.editor.IStandaloneEditorConstructionOptions
  markers?: Marker[]
}) {
  const onChangeRef = useRef<((_: string) => void) | undefined>(null)
  const onSaveRef = useRef<((_: string) => void) | undefined>(null)
  const divRef = useRef<HTMLDivElement>(null)
  const autoFocusRef = useRef(autoFocus)
  const settingsInitial = useRef(settings)
  const editorRef = useRef<monaco.editor.IStandaloneCodeEditor>(null)
  const fixesRef = useRef<Map<string, Fix>>(new Map())

  // TODO: replace with useEventEffect
  useEffect(() => {
    onChangeRef.current = onChange
  }, [onChange])
  useEffect(() => {
    onSaveRef.current = onSave
  }, [onSave])

  useEffect(() => {
    if (markers == null) {
      return
    }

    const fixesMap = new Map<string, Fix>()
    for (const marker of markers) {
      if (marker.fix) {
        const key = createMarkerKey(marker)
        fixesMap.set(key, marker.fix)
      }
    }
    fixesRef.current = fixesMap

    const model = editorRef.current?.getModel()
    if (model != null) {
      monaco.editor.setModelMarkers(model, "squawk", markers)
    }
  }, [markers])

  useLayoutEffect(() => {
    const editor = monaco.editor.create(
      divRef.current!,
      settingsInitial.current,
    )
    editor.addCommand(monaco.KeyMod.CtrlCmd | monaco.KeyCode.KeyS, () =>
      onSaveRef.current?.(editor.getValue()),
    )
    editor.onDidBlurEditorText(() => {
      onSaveRef.current?.(editor.getValue())
    })
    monaco.languages.register({ id: "rast" })
    const tokenProvider = monaco.languages.setMonarchTokensProvider("rast", {
      tokenizer: {
        // via: https://github.com/rust-lang/rust-analyzer/blob/9691da7707ea7c50922fe1647b1c2af47934b9fa/editors/code/ra_syntax_tree.tmGrammar.json#L16C17-L16C17
        root: [
          // Node type (entity.name.class)
          [/^[\s]*([A-Z_][A-Z_0-9]*?)@/, "entity.identifier.type"],

          // Node range index (constant.numeric)
          [/\d+/, "number"],

          // Token text (string)
          [/"[^"]*"/, "string"],
        ],
      },
    })

    const codeActionProvider = monaco.languages.registerCodeActionProvider(
      "pgsql",
      {
        provideCodeActions: (model, _range, context) => {
          const actions: monaco.languages.CodeAction[] = []
          for (const marker of context.markers) {
            if (marker.source === "squawk") {
              const key = createMarkerKey(marker)
              const fix = fixesRef.current.get(key)
              if (fix) {
                const edits = fix.edits.map(
                  (edit): monaco.languages.IWorkspaceTextEdit => {
                    return {
                      resource: model.uri,
                      versionId: model.getVersionId(),
                      textEdit: {
                        range: new monaco.Range(
                          edit.start_line_number + 1,
                          edit.start_column + 1,
                          edit.end_line_number + 1,
                          edit.end_column + 1,
                        ),
                        text: edit.text,
                      },
                    }
                  },
                )
                actions.push({
                  title: fix.title,
                  diagnostics: [marker],
                  kind: "quickfix",
                  edit: {
                    edits,
                  },
                  isPreferred: true,
                })
              }
            }
          }

          return {
            actions,
            dispose: () => {},
          }
        },
      },
    )

    editor.onDidChangeModelContent(() => {
      onChangeRef.current?.(editor.getValue())
    })
    if (autoFocusRef.current) {
      editor.focus()
    }
    editorRef.current = editor
    return () => {
      editorRef.current = null
      codeActionProvider.dispose()
      editor?.dispose()
      tokenProvider.dispose()
    }
  }, [])
  useEffect(() => {
    if (value != null) {
      editorRef.current?.setValue(value)
    }
  }, [value])

  return (
    <div
      ref={divRef}
      className="w-full [max-height:calc(100vh_-_30px)] h-[50vh] md:h-full"
    />
  )
}

// I thought if we defined the numeric values for the variants the bindgen would use them, but it doesn't
// https://github.com/rustwasm/wasm-bindgen/issues/2407
function convertSeverity(x: LintError["severity"]): monaco.MarkerSeverity {
  switch (x) {
    case "Error":
      return monaco.MarkerSeverity.Error
    case "Warning":
      return monaco.MarkerSeverity.Warning
    case "Info":
      return monaco.MarkerSeverity.Info
    case "Hint":
      return monaco.MarkerSeverity.Hint
  }
}

type Marker = monaco.editor.IMarkerData & {
  id: string
  range_start: number
  range_end: number
  messages: string[]
  fix?: Fix
}

function createMarkerKey(marker: {
  startLineNumber: number
  startColumn: number
  endLineNumber: number
  endColumn: number
  message: string
}): string {
  // TODO: probably a better way to do this
  return `${marker.startLineNumber}:${marker.startColumn}:${marker.endLineNumber}:${marker.endColumn}:${marker.message}`
}

function SyntaxTreePanel({ text }: { text: string }) {
  const value = useDumpCst(text)
  return (
    <Editor
      value={value}
      settings={{
        ...SETTINGS,
        fontSize: 14,
        value,
        language: "rast",
        readOnly: true,
        lineNumbers: "off",
      }}
    />
  )
}

function useMarkers(text: string): Array<Marker> {
  const errors = useErrors(text)
  return errors.map((x): Marker => {
    const startLineNumber = x.start_line_number + 1
    const startColumn = x.start_column + 1
    const endLineNumber = x.end_line_number + 1
    let endColumn = x.end_column + 1
    // parser will return zero length errors for things like trailing semicolon,
    // we probably want to fix this, but for now we compensate by setting the
    // min length of the marker to 1
    if (endColumn === startColumn) {
      endColumn += 1
    }
    return {
      severity: convertSeverity(x.severity),
      id: `${x.start_line_number}${x.start_column}${x.end_line_number}${x.end_column}${x.severity}${x.code}`,
      startLineNumber,
      startColumn,
      endLineNumber,
      endColumn,
      range_start: x.range_start,
      range_end: x.range_end,
      messages: x.messages,
      fix: x.fix,
      code: {
        value: x.code,
        target: monaco.Uri.parse(
          `https://squawkhq.com/docs/${encodeURIComponent(x.code)}/`,
        ),
      },
      // doesn't support markdown -- vscode does tho :/
      // https://github.com/microsoft/monaco-editor/issues/1264
      // https://stackoverflow.com/questions/62362741/using-markdown-in-imarkerdata
      message: x.message,
      source: "squawk",
    }
  })
}

function ErrorList({ errors }: { errors: Marker[] }) {
  if (errors.length === 0) {
    return <div>no errors!</div>
  }
  return errors.map((x) => {
    const color =
      x.severity === monaco.MarkerSeverity.Warning
        ? "border-l-amber-300"
        : x.severity === monaco.MarkerSeverity.Error
          ? "border-l-red-400"
          : ""
    const code = typeof x.code === "string" ? x.code : x.code?.value
    return (
      <div className={`${color} border-l-2 pl-2 leading-5`} key={x.id}>
        <div data-range={`${x.range_start}..${x.range_end}`}>
          {code == null ? (
            <div className="font-semibold">{code}</div>
          ) : (
            <a
              href={`https://squawkhq.com/docs/${encodeURIComponent(code)}`}
              target="_blank"
            >
              {code}
            </a>
          )}
          :{x.startLineNumber}:{x.startColumn}: {x.message}
        </div>
        {x.messages.length > 0 && (
          <div className="pl-4 pt-1">
            {x.messages.map((note) => {
              return (
                <div key={note}>
                  <span className="text-blue-400 font-semibold">help:</span>{" "}
                  {note}
                </div>
              )
            })}
          </div>
        )}
      </div>
    )
  })
}

function ErrorPanel({ errors }: { errors: Marker[] }) {
  return (
    <div className="w-full [max-height:calc(100vh_-_30px)] h-[50vh] md:h-full bg-[rgb(30,30,30)] p-4 font-mono flex flex-col gap-4 overflow-auto text-white text-sm select-auto">
      <ErrorList errors={errors} />
    </div>
  )
}

function Nav({ children }: { children: React.ReactNode }) {
  return (
    <nav className="flex items-center justify-between px-4 py-2 bg-[rgb(30,30,30)] text-white border-b border-[#282828] pb-1 cursor-default">
      <div className="flex items-center space-x-4">
        <div className="flex items-center space-x-2">
          <img src="/owl.png" alt="Squawk Owl Logo" width="24" height="24" />
          <h1 className="text-lg font-semibold">Squawk Playground</h1>
        </div>
        <div className="flex space-x-2">{children}</div>
      </div>
    </nav>
  )
}

function handleSave(text: string) {
  const encoded = compressToEncodedURIComponent(text)
  window.location.hash = `code/${encoded}`
  navigator.clipboard
    .writeText(window.location.href)
    .then(() => {
      console.log("foo")
    })
    .catch((err) => {
      console.log(err)
    })

  try {
    localStorage.setItem(STORAGE_KEY, compress(text))
  } catch {
    // pass
  }
}

function ShareButton({ text }: { text: string }) {
  return (
    <button
      className="px-3 py-1 rounded hover:bg-gray-700 transition-colors"
      onClick={() => {
        handleSave(text)
      }}
    >
      Share
    </button>
  )
}
