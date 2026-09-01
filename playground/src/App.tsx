import {
  useState,
  useEffect,
  useLayoutEffect,
  useRef,
  useEffectEvent,
} from "react"
import * as monaco from "monaco-editor"
import * as stylex from "@stylexjs/stylex"
import {
  LintError,
  Fix,
  useDumpCst,
  useDumpTokens,
  useErrors,
  useFormat,
} from "./squawk"
import {
  compress,
  compressToEncodedURIComponent,
  decompress,
  decompressFromEncodedURIComponent,
} from "lz-string"
import {
  ideCodeActions,
  provideInlayHints,
  provideHover,
  provideDefinition,
  provideReferences,
  provideDocumentSymbols,
  provideFoldingRanges,
  provideSelectionRanges,
  provideCompletionItems,
  semanticTokensProvider,
} from "./providers"
import { language as pgsqlMonarchLanguage } from "./pgsql"
import { breakpoints, colors, transitions } from "./tokens.stylex"
import BUILTINS_SQL from "./builtins.sql?raw"

const modes = ["Lint", "Format", "Syntax Tree", "Tokens"] as const
const STORAGE_KEY = "playground-history-v3"

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

select *, now() from users;
`

const SETTINGS = {
  value: DEFAULT_CONTENT,
  language: "pgsql",
  tabSize: 2,
  insertSpaces: true,
  detectIndentation: false,
  theme: "squawk-dark",
  minimap: { enabled: false },
  automaticLayout: true,
  scrollBeyondLastLine: false,
  folding: true,
  showFoldingControls: "mouseover",
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
  "semanticHighlighting.enabled": true,
} satisfies monaco.editor.IStandaloneEditorConstructionOptions

const styles = stylex.create({
  app: {
    display: "flex",
    flexDirection: "column",
    height: "100vh",
  },
  nav: {
    display: "flex",
    alignItems: "center",
    justifyContent: "space-between",
    paddingInline: "1rem",
    paddingTop: "0.5rem",
    paddingBottom: "0.25rem",
    backgroundColor: colors.background,
    color: colors.text,
    borderBottomWidth: "1px",
    borderBottomColor: colors.border,
    cursor: "default",
  },
  navGroup: {
    display: "flex",
    alignItems: "center",
    gap: "1rem",
  },
  navTitle: {
    display: "flex",
    alignItems: "center",
    gap: "0.5rem",
  },
  navLinks: {
    display: "flex",
    gap: "0.5rem",
  },
  heading: {
    fontSize: "1.125rem",
    lineHeight: "1.75rem",
    fontWeight: 600,
  },
  navItem: {
    paddingInline: "0.75rem",
    paddingBlock: "0.25rem",
    borderRadius: "0.25rem",
    backgroundColor: {
      default: "transparent",
      ":hover": colors.hover,
    },
    transitionProperty: transitions.colors,
    transitionDuration: transitions.duration,
    transitionTimingFunction: transitions.easing,
  },
  main: {
    display: "flex",
    flexGrow: 1,
    flexShrink: 1,
    flexBasis: "0%",
    marginTop: "0.25rem",
  },
  panels: {
    display: "grid",
    gridTemplateColumns: "repeat(1, minmax(0, 1fr))",
    flexGrow: 1,
    flexShrink: 1,
    flexBasis: "0%",
    overflow: "auto",
  },
  panelsSplit: {
    gridTemplateColumns: {
      default: "repeat(1, minmax(0, 1fr))",
      [breakpoints.md]: "repeat(2, minmax(0, 1fr))",
    },
  },
  column: {
    display: "flex",
    flexDirection: "column",
    flexGrow: 1,
    flexShrink: 1,
    flexBasis: "0%",
  },
  panel: {
    width: "100%",
    maxHeight: "calc(100vh - 30px)",
    height: {
      default: "50vh",
      [breakpoints.md]: "100%",
    },
  },
  banner: {
    paddingInline: "0.75rem",
    paddingBlock: "0.5rem",
    fontSize: "0.875rem",
    lineHeight: "1.25rem",
    color: colors.bannerText,
  },
  bannerRow: {
    display: "flex",
    alignItems: "center",
    justifyContent: "space-between",
  },
  bannerInfo: {
    backgroundColor: colors.info,
  },
  bannerWarning: {
    backgroundColor: colors.warningBackground,
  },
  bannerButton: {
    paddingInline: "0.5rem",
    paddingBlock: "0.25rem",
    borderWidth: "1px",
    borderColor: colors.bannerBorder,
    borderRadius: "0.25rem",
    backgroundColor: {
      default: "transparent",
      ":hover": colors.accent,
    },
  },
  controls: {
    paddingInline: "1rem",
    paddingBlock: "0.5rem",
    backgroundColor: colors.background,
    borderLeftWidth: "1px",
    borderLeftColor: colors.border,
  },
  controlsList: {
    display: "flex",
    flexDirection: "column",
    gap: "0.25rem",
  },
  modeButton: {
    width: "100%",
    paddingInline: "0.5rem",
    paddingBlock: "0.25rem",
    fontSize: "0.875rem",
    lineHeight: "1.25rem",
    borderRadius: "0.25rem",
    transitionProperty: transitions.colors,
    transitionDuration: transitions.duration,
    transitionTimingFunction: transitions.easing,
  },
  modeButtonActive: {
    backgroundColor: colors.accent,
    color: colors.text,
  },
  modeButtonInactive: {
    color: colors.subtleText,
    backgroundColor: {
      default: "transparent",
      ":hover": colors.hover,
    },
  },
  errorPanel: {
    display: "flex",
    flexDirection: "column",
    gap: "1rem",
    overflow: "auto",
    padding: "1rem",
    backgroundColor: colors.background,
    color: colors.text,
    fontFamily:
      'ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, "Liberation Mono", "Courier New", monospace',
    fontSize: "0.875rem",
    lineHeight: "1.25rem",
    userSelect: "auto",
  },
  error: {
    paddingLeft: "0.5rem",
    borderLeftWidth: "2px",
    borderLeftColor: "currentColor",
    lineHeight: "1.25rem",
  },
  errorWarning: {
    borderLeftColor: colors.warning,
  },
  errorError: {
    borderLeftColor: colors.error,
  },
  errorCode: {
    fontWeight: 600,
  },
  errorNotes: {
    paddingLeft: "1rem",
    paddingTop: "0.25rem",
  },
  errorNoteLabel: {
    color: colors.infoText,
    fontWeight: 600,
  },
})

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

// pulling this into its own function due to gap in react compiler
// https://github.com/facebook/react/issues/34761
// this avoid a de-opt
function saveMode(mode: Mode | null) {
  try {
    localStorage.setItem("play-mode-v1", mode ?? "none")
  } catch {
    // pass
  }
}

function useMode() {
  const [mode, setActiveMode] = useState<Mode | null>(() => initialMode())

  useEffect(() => {
    saveMode(mode)
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
  const [version, setVersion] = useState(0)
  const [file, setFile] = useState<"current" | "builtins">("current")
  const editorRef = useRef<monaco.editor.IStandaloneCodeEditor | null>(null)

  const markers = useMarkers(text, version)

  return (
    <div {...stylex.props(styles.app)}>
      <Nav>
        <a
          href="https://squawkhq.com"
          target="_blank"
          {...stylex.props(styles.navItem)}
        >
          Docs
        </a>
        <a
          href="https://squawkhq.com/docs/rules"
          target="_blank"
          {...stylex.props(styles.navItem)}
        >
          Rules
        </a>
        <a
          href="https://github.com/sbdchd/squawk"
          target="_blank"
          {...stylex.props(styles.navItem)}
        >
          GitHub
        </a>
        <ShareButton text={text} />
      </Nav>
      <div {...stylex.props(styles.main)}>
        <div
          {...stylex.props(styles.panels, mode != null && styles.panelsSplit)}
        >
          <div {...stylex.props(styles.column)}>
            {file == "builtins" ? (
              <BuiltinsBanner
                onBack={() => {
                  // TODO: Might want to use an imperative ref so we can move this into the Editor
                  editorRef.current?.setModel(getCurrentModel())
                  editorRef.current?.updateOptions({ readOnly: false })
                }}
              />
            ) : null}
            <Editor
              onChange={(text, version) => {
                setState(text)
                setVersion(version)
              }}
              autoFocus
              markers={markers}
              settings={{ ...SETTINGS, value: text }}
              onSave={handleSave}
              onModelChange={(model) => {
                if (model?.path === builtinsUri.path) {
                  setFile("builtins")
                } else if (model?.path === currentUri.path) {
                  setFile("current")
                }
              }}
              ref={editorRef}
            />
          </div>
          {mode === "Syntax Tree" ? (
            // TODO: it might be better to have an editor and switch the underlying monaco models
            <SyntaxTreePanel text={text} version={version} />
          ) : mode === "Tokens" ? (
            <TokenPanel text={text} version={version} />
          ) : mode === "Lint" ? (
            <ErrorPanel errors={markers} />
          ) : mode === "Format" ? (
            <FormatPanel text={text} version={version} />
          ) : mode == null ? null : (
            assertNever(mode)
          )}
        </div>
        <Controls activeMode={mode} onModeChange={setActiveMode} />
      </div>
    </div>
  )
}

function BuiltinsBanner({ onBack }: { onBack: () => void }) {
  return (
    <div {...stylex.props(styles.banner, styles.bannerRow, styles.bannerInfo)}>
      <div>Viewing postgres stubs (read-only)</div>
      <button {...stylex.props(styles.bannerButton)} onClick={onBack}>
        Go Back
      </button>
    </div>
  )
}

function TokenPanel({ text, version }: { text: string; version: number }) {
  const value = useDumpTokens(text, version)
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

function FormatPanel({ text, version }: { text: string; version: number }) {
  const result = useFormat(text, version)
  const value = result.ok ? result.text : result.error
  return (
    <div {...stylex.props(styles.column)}>
      {result.ok ? (
        <Editor
          value={value}
          settings={{
            ...SETTINGS,
            value,
            language: "pgsql-formatted",
            readOnly: true,
          }}
        />
      ) : (
        <div {...stylex.props(styles.banner, styles.bannerWarning)}>
          Error formatting this file.
        </div>
      )}
    </div>
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
    <div {...stylex.props(styles.controls)}>
      <div {...stylex.props(styles.controlsList)}>
        {modes.map((mode) => (
          <button
            key={mode}
            onClick={() => {
              onModeChange(activeMode === mode ? null : mode)
            }}
            {...stylex.props(
              styles.modeButton,
              activeMode === mode
                ? styles.modeButtonActive
                : styles.modeButtonInactive,
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

const pgsqlConfig: monaco.languages.LanguageConfiguration = {
  comments: {
    lineComment: "--",
    blockComment: ["/*", "*/"],
  },
  brackets: [
    ["{", "}"],
    ["[", "]"],
    ["(", ")"],
  ],
  autoClosingPairs: [
    { open: "{", close: "}" },
    { open: "[", close: "]" },
    { open: "(", close: ")" },
    { open: '"', close: '"', notIn: ["string"] },
    { open: "$$", close: "$$", notIn: ["string", "comment"] },
    { open: "E'", close: "'", notIn: ["string", "comment"] },
    { open: "e'", close: "'", notIn: ["string", "comment"] },
    { open: "U&'", close: "'", notIn: ["string", "comment"] },
    { open: "u&'", close: "'", notIn: ["string", "comment"] },
    { open: "B'", close: "'", notIn: ["string", "comment"] },
    { open: "b'", close: "'", notIn: ["string", "comment"] },
    { open: "X'", close: "'", notIn: ["string", "comment"] },
    { open: "x'", close: "'", notIn: ["string", "comment"] },
    { open: "N'", close: "'", notIn: ["string", "comment"] },
    { open: "'", close: "'", notIn: ["string", "comment"] },
    { open: "/*", close: " */", notIn: ["string", "comment"] },
  ],
  surroundingPairs: [
    { open: "{", close: "}" },
    { open: "[", close: "]" },
    { open: "(", close: ")" },
    { open: '"', close: '"' },
    { open: "'", close: "'" },
    { open: "`", close: "`" },
    { open: "$$", close: "$$" },
  ],
  onEnterRules: [
    {
      beforeText: /^\s*--.*$/,
      afterText: /\S/,
      action: {
        indentAction: monaco.languages.IndentAction.None,
        appendText: "-- ",
      },
    },
    {
      beforeText: /^\s*\/\*/,
      afterText: /^\s*\*\/$/,
      action: {
        indentAction: monaco.languages.IndentAction.IndentOutdent,
        appendText: " * ",
      },
    },
    {
      beforeText: /^\s*\/\*(?!.*\*\/).*$/,
      action: {
        indentAction: monaco.languages.IndentAction.None,
        appendText: " * ",
      },
    },
    {
      beforeText: /^\s*\*(?!\/).*$/,
      action: {
        indentAction: monaco.languages.IndentAction.None,
        appendText: "* ",
      },
    },
  ],
}

let monacoGlobalProvidersRegistered = false
// Only want to register these once, otherwise we'll end up with multiple
// providers running and get dupe results for things like hover
function registerMonacoProvidersOnce() {
  if (monacoGlobalProvidersRegistered) {
    return
  }
  monacoGlobalProvidersRegistered = true
  // vs-dark maps variable to a blue color which makes everything look like a
  // keyword. So we use white instead which was what the `foo` in `select 1 foo`
  // was before semantic syntax highlighting.
  monaco.editor.defineTheme("squawk-dark", {
    base: "vs-dark",
    inherit: true,
    rules: [{ token: "variable", foreground: "D4D4D4" }],
    colors: {},
  })
  const languageConfig = monaco.languages.setLanguageConfiguration(
    "pgsql",
    pgsqlConfig,
  )
  const pgsqlTokenProvider = monaco.languages.setMonarchTokensProvider(
    "pgsql",
    pgsqlMonarchLanguage,
  )

  monaco.languages.register({ id: "pgsql-formatted" })
  const pgsqlFormattedTokenProvider = monaco.languages.setMonarchTokensProvider(
    "pgsql-formatted",
    pgsqlMonarchLanguage,
  )

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
      provideCodeActions: (model, range, context) => {
        const actions: monaco.languages.CodeAction[] = ideCodeActions(
          model,
          range,
        )
        for (const marker of context.markers) {
          if (marker.source === "squawk") {
            const key = createMarkerKey(marker)
            const fix = fixesRef.get(key)
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

  const hoverProvider = monaco.languages.registerHoverProvider("pgsql", {
    provideHover,
  })

  const definitionProvider = monaco.languages.registerDefinitionProvider(
    "pgsql",
    {
      provideDefinition,
    },
  )

  const referencesProvider = monaco.languages.registerReferenceProvider(
    "pgsql",
    {
      provideReferences,
    },
  )

  const documentSymbolProvider =
    monaco.languages.registerDocumentSymbolProvider("pgsql", {
      provideDocumentSymbols,
    })

  const inlayHintsProvider = monaco.languages.registerInlayHintsProvider(
    "pgsql",
    {
      provideInlayHints,
    },
  )

  const foldingRangeProvider = monaco.languages.registerFoldingRangeProvider(
    "pgsql",
    {
      provideFoldingRanges,
    },
  )

  const selectionRangeProvider =
    monaco.languages.registerSelectionRangeProvider("pgsql", {
      provideSelectionRanges,
    })

  const completionProvider = monaco.languages.registerCompletionItemProvider(
    "pgsql",
    {
      triggerCharacters: ["."],
      provideCompletionItems,
    },
  )

  const documentSemanticTokensProvider =
    monaco.languages.registerDocumentSemanticTokensProvider(
      "pgsql",
      semanticTokensProvider,
    )

  return () => {
    languageConfig.dispose()
    pgsqlTokenProvider.dispose()
    pgsqlFormattedTokenProvider.dispose()
    codeActionProvider.dispose()
    hoverProvider.dispose()
    definitionProvider.dispose()
    referencesProvider.dispose()
    documentSymbolProvider.dispose()
    foldingRangeProvider.dispose()
    inlayHintsProvider.dispose()
    selectionRangeProvider.dispose()
    completionProvider.dispose()
    documentSemanticTokensProvider.dispose()
    tokenProvider.dispose()
  }
}

const builtinsUri = monaco.Uri.parse("file:///builtins.sql")
const currentUri = monaco.Uri.parse("file:///current.sql")

function getBuiltinsModel() {
  let builtinsModel = monaco.editor.getModel(builtinsUri)
  if (!builtinsModel) {
    builtinsModel = monaco.editor.createModel(
      BUILTINS_SQL,
      "pgsql",
      builtinsUri,
    )
  }
  return builtinsModel
}

function getCurrentModel(defaultText?: string | undefined) {
  let currentModel = monaco.editor.getModel(currentUri)
  if (!currentModel) {
    currentModel = monaco.editor.createModel(
      defaultText ?? "",
      "pgsql",
      currentUri,
    )
  }
  return currentModel
}

// TODO: this is hacky
let fixesRef: Map<string, Fix> = new Map()

function Editor({
  onChange,
  autoFocus,
  settings,
  value,
  markers,
  onSave,
  onModelChange,
  ref,
}: {
  value?: string
  autoFocus?: boolean
  onChange?: (_: string, version: number) => void
  onSave?: (_: string) => void
  settings: monaco.editor.IStandaloneEditorConstructionOptions
  markers?: Marker[]
  onModelChange?: (uri: monaco.Uri | null) => void
  ref?: React.RefObject<monaco.editor.IStandaloneCodeEditor | null>
}) {
  const onChangeText = useEffectEvent((text: string, version: number) => {
    onChange?.(text, version)
  })
  const onSaveText = useEffectEvent((text: string) => {
    onSave?.(text)
  })
  const onModelChange_ = useEffectEvent((uri: monaco.Uri | null) => {
    onModelChange?.(uri)
  })
  const divRef = useRef<HTMLDivElement>(null)
  const autoFocusRef = useRef(autoFocus)
  const settingsInitial = useRef(settings)
  const editorRef = useRef<monaco.editor.IStandaloneCodeEditor>(null)

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
    fixesRef = fixesMap

    const model = editorRef.current?.getModel()
    if (model != null) {
      monaco.editor.setModelMarkers(model, "squawk", markers)
    }
  }, [markers])

  useLayoutEffect(() => {
    registerMonacoProvidersOnce()
    const editor = monaco.editor.create(
      divRef.current!,
      settingsInitial.current,
    )
    if (ref) {
      ref.current = editor
    }
    if (!editor.getOption(monaco.editor.EditorOption.readOnly)) {
      editor.setModel(getCurrentModel(settingsInitial.current?.value))
    }
    editor.addCommand(monaco.KeyMod.CtrlCmd | monaco.KeyCode.KeyS, () =>
      onSaveText(editor.getValue()),
    )

    editor.onDidChangeModel((e) => {
      onModelChange_(e.newModelUrl)
    })

    const opener = monaco.editor.registerEditorOpener({
      openCodeEditor: (
        editor: monaco.editor.ICodeEditor,
        resource: monaco.Uri,
        selectionOrPosition?: monaco.IRange | monaco.IPosition,
      ): boolean | Promise<boolean> => {
        if (editor.getOption(monaco.editor.EditorOption.readOnly)) {
          return false
        }
        let switched = false
        if (resource.path === builtinsUri.path) {
          editor.setModel(getBuiltinsModel())
          editor.updateOptions({ readOnly: true })
          switched = true
        } else if (resource.path === currentUri.path) {
          // we're already in the "current" file
          switched = true
        }

        if (switched && selectionOrPosition) {
          if ("startLineNumber" in selectionOrPosition) {
            editor.setSelection(selectionOrPosition)
            editor.revealRangeInCenter(selectionOrPosition)
          } else {
            editor.setPosition(selectionOrPosition)
            editor.revealPositionInCenter(selectionOrPosition)
          }
          editor.focus()
          return true
        }

        return false
      },
    })

    editor.onDidChangeModelContent(() => {
      onChangeText(editor.getValue(), editor.getModel()?.getVersionId() ?? 0)
    })
    if (autoFocusRef.current) {
      editor.focus()
    }
    editorRef.current = editor
    return () => {
      editorRef.current = null

      editor?.dispose()
      opener.dispose()
    }
  }, [ref])
  useEffect(() => {
    if (value != null) {
      editorRef.current?.setValue(value)
    }
  }, [value])

  return <div ref={divRef} {...stylex.props(styles.panel)} />
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

function SyntaxTreePanel({ text, version }: { text: string; version: number }) {
  const value = useDumpCst(text, version)
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

function useMarkers(text: string, version: number): Array<Marker> {
  const errors = useErrors(text, version)
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
    const severity =
      x.severity === monaco.MarkerSeverity.Warning
        ? styles.errorWarning
        : x.severity === monaco.MarkerSeverity.Error
          ? styles.errorError
          : null
    const code = typeof x.code === "string" ? x.code : x.code?.value
    return (
      <div {...stylex.props(styles.error, severity)} key={x.id}>
        <div data-range={`${x.range_start}..${x.range_end}`}>
          {code == null ? (
            <div {...stylex.props(styles.errorCode)}>{code}</div>
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
          <div {...stylex.props(styles.errorNotes)}>
            {x.messages.map((note) => {
              return (
                <div key={note}>
                  <span {...stylex.props(styles.errorNoteLabel)}>help:</span>{" "}
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
    <div {...stylex.props(styles.panel, styles.errorPanel)}>
      <ErrorList errors={errors} />
    </div>
  )
}

function Nav({ children }: { children: React.ReactNode }) {
  return (
    <nav {...stylex.props(styles.nav)}>
      <div {...stylex.props(styles.navGroup)}>
        <div {...stylex.props(styles.navTitle)}>
          <img src="/owl.png" alt="Squawk Owl Logo" width="24" height="24" />
          <h1 {...stylex.props(styles.heading)}>Squawk Playground</h1>
        </div>
        <div {...stylex.props(styles.navLinks)}>{children}</div>
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
      {...stylex.props(styles.navItem)}
      onClick={() => {
        handleSave(text)
      }}
    >
      Share
    </button>
  )
}
