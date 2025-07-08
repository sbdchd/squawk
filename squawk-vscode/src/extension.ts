import * as vscode from "vscode"
import { execFileSync } from "child_process"
import {
  LanguageClient,
  LanguageClientOptions,
  Executable,
  ServerOptions,
  State,
  StateChangeEvent,
} from "vscode-languageclient/node"

// via rust-analyzer
// https://github.com/rust-lang/rust-analyzer/blob/f14bf95931f17ae1830a77d6f0dff38cabb401da/editors/code/src/util.ts#L157C1-L157C1
class LazyOutputChannel implements vscode.OutputChannel {
  constructor(name: string) {
    this.name = name
  }

  name: string
  _channel: vscode.OutputChannel | undefined

  get channel(): vscode.OutputChannel {
    if (!this._channel) {
      this._channel = vscode.window.createOutputChannel(this.name)
    }
    return this._channel
  }

  append(value: string): void {
    this.channel.append(value)
  }

  appendLine(value: string): void {
    this.channel.appendLine(value)
  }

  replace(value: string): void {
    this.channel.replace(value)
  }

  clear(): void {
    if (this._channel) {
      this._channel.clear()
    }
  }

  show(
    columnOrPreserveFocus?: vscode.ViewColumn | boolean,
    preserveFocus?: boolean,
  ): void {
    if (typeof columnOrPreserveFocus === "boolean") {
      this.channel.show(columnOrPreserveFocus)
    } else {
      this.channel.show(columnOrPreserveFocus, preserveFocus)
    }
  }

  hide(): void {
    if (this._channel) {
      this._channel.hide()
    }
  }

  dispose(): void {
    if (this._channel) {
      this._channel.dispose()
    }
  }
}

let client: LanguageClient | undefined
let log: Pick<
  vscode.LogOutputChannel,
  "trace" | "debug" | "info" | "warn" | "error" | "show"
>
const onClientStateChange = new vscode.EventEmitter<StateChangeEvent>()

export async function activate(context: vscode.ExtensionContext) {
  log = vscode.window.createOutputChannel("Squawk Client", {
    log: true,
  })

  log.info("Squawk activate")

  const syntaxTreeProvider = new SyntaxTreeProvider(context)
  context.subscriptions.push(
    vscode.workspace.registerTextDocumentContentProvider(
      "squawk-syntax-tree",
      syntaxTreeProvider,
    ),
  )

  const tokensProvider = new TokensProvider(context)
  context.subscriptions.push(
    vscode.workspace.registerTextDocumentContentProvider(
      "squawk-tokens",
      tokensProvider,
    ),
  )

  context.subscriptions.push(
    vscode.commands.registerCommand("squawk.serverVersion", () => {
      try {
        const serverPath = getSquawkPath(context)
        const stdout = execFileSync(serverPath.path, ["--version"], {
          encoding: "utf8",
        })
        const version = stdout.trim()
        vscode.window.showInformationMessage(
          `Squawk Server Version: ${version}`,
        )
        return version
      } catch (error) {
        vscode.window.showErrorMessage(
          `Failed to get server version: ${String(error)}`,
        )
      }
    }),
  )

  setupStatusBarItem(context)

  context.subscriptions.push(
    vscode.commands.registerCommand("squawk.showLogs", () => {
      client?.outputChannel?.show()
    }),
  )

  context.subscriptions.push(
    vscode.commands.registerCommand("squawk.showClientLogs", () => {
      log.show()
    }),
  )

  context.subscriptions.push(
    vscode.commands.registerCommand("squawk.startServer", async () => {
      await startServer(context)
    }),
  )

  context.subscriptions.push(
    vscode.commands.registerCommand("squawk.stopServer", async () => {
      await stopServer()
    }),
  )

  context.subscriptions.push(onClientStateChange)

  await startServer(context)
}

export async function deactivate() {}

function isSqlDocument(document: vscode.TextDocument): boolean {
  return document.languageId === "sql" || document.languageId === "postgres"
}

function isSqlEditor(editor: vscode.TextEditor): boolean {
  return isSqlDocument(editor.document)
}

function setupStatusBarItem(context: vscode.ExtensionContext) {
  const statusBarItem = vscode.window.createStatusBarItem(
    vscode.StatusBarAlignment.Left,
  )
  statusBarItem.text = "Squawk"
  statusBarItem.command = "squawk.showLogs"
  context.subscriptions.push(statusBarItem)

  const onDidChangeActiveTextEditor = (
    editor: vscode.TextEditor | undefined,
  ) => {
    if (editor && isSqlEditor(editor)) {
      updateStatusBarItem(statusBarItem)
      statusBarItem.show()
    } else {
      statusBarItem.hide()
    }
  }

  onDidChangeActiveTextEditor(vscode.window.activeTextEditor)

  context.subscriptions.push(
    vscode.window.onDidChangeActiveTextEditor((editor) => {
      onDidChangeActiveTextEditor(editor)
    }),
  )

  context.subscriptions.push(
    onClientStateChange.event(() => {
      updateStatusBarItem(statusBarItem)
    }),
  )
}

function updateStatusBarItem(statusBarItem: vscode.StatusBarItem) {
  if (!client) {
    return
  }
  let statusText: string
  let icon: string
  let backgroundColor: vscode.ThemeColor | undefined
  switch (client.state) {
    case State.Stopped:
      statusText = "Stopped"
      icon = "$(stop-circle) "
      backgroundColor = new vscode.ThemeColor("statusBarItem.warningBackground")
      break
    case State.Starting:
      statusText = "Starting..."
      icon = "$(loading~spin) "
      backgroundColor = undefined
      break
    case State.Running:
      statusText = "Running"
      icon = ""
      backgroundColor = undefined
      break
    default:
      assertNever(client.state)
  }

  statusBarItem.text = `${icon}Squawk`
  statusBarItem.backgroundColor = backgroundColor
  statusBarItem.tooltip = `Status: ${statusText}\nClick to show server logs`
}

function getSquawkPath(context: vscode.ExtensionContext): vscode.Uri {
  const ext = process.platform === "win32" ? ".exe" : ""
  return vscode.Uri.joinPath(context.extensionUri, "server", `squawk${ext}`)
}

async function startServer(context: vscode.ExtensionContext) {
  const state = client?.state
  switch (state) {
    case State.Running:
    case State.Starting:
      log.info("Server is already running")
      break
    case State.Stopped:
    case undefined:
      break
    default:
      assertNever(state)
  }

  log.info("Starting Squawk Language Server...")

  const squawkPath = getSquawkPath(context)
  const hasBinary = await vscode.workspace.fs.stat(squawkPath).then(
    () => true,
    () => false,
  )
  if (!hasBinary) {
    const errorMsg = `Squawk binary not found at: ${squawkPath.path}`
    log.error(`ERROR: ${errorMsg}`)
    vscode.window.showErrorMessage(errorMsg)
    return
  }
  log.info(`Found Squawk binary at: ${squawkPath.path}`)

  const serverExecutable: Executable = {
    command: squawkPath.path,
    args: ["server", "--verbose"],
  }
  const serverOptions: ServerOptions = serverExecutable
  const clientOptions: LanguageClientOptions = {
    documentSelector: [{ language: "sql" }, { language: "postgres" }],
    traceOutputChannel: new LazyOutputChannel("Squawk Trace"),
    outputChannel: vscode.window.createOutputChannel("Squawk Language Server"),
  }
  client = new LanguageClient(
    "squawk",
    "Squawk Language Server",
    serverOptions,
    clientOptions,
  )

  context.subscriptions.push(
    client.onDidChangeState((event) => {
      onClientStateChange.fire(event)
    }),
  )
  context.subscriptions.push(client)

  log.info("server starting...")
  try {
    await client.start()
    log.info("server started successfully")
  } catch (error) {
    log.error("Failed to start server:", error)
    vscode.window.showErrorMessage(`Failed to start server: ${String(error)}`)
  }
}

async function stopServer() {
  if (!client) {
    log.info("No client to stop server")
    return
  }

  if (client.state === State.Stopped) {
    log.info("Server is already stopped")
    return
  }

  log.info("Stopping server...")

  await client.stop()

  log.info("server stopped")
}

// Based on rust-analyzer's SyntaxTree support:
// https://github.com/rust-lang/rust-analyzer/blob/c0eaff7dd1fdfffed4e5706780e79967760d1d9b/editors/code/src/commands.ts#L432-L510
class SyntaxTreeProvider implements vscode.TextDocumentContentProvider {
  _eventEmitter = new vscode.EventEmitter<vscode.Uri>()
  _activeEditor: vscode.TextEditor | undefined
  // TODO: for now we only show syntax highlighting when someone has
  // rust-analyzer installed which ships with the rast grammar
  _uri = vscode.Uri.parse("squawk-syntax-tree://syntaxtree/tree.rast")

  constructor(context: vscode.ExtensionContext) {
    context.subscriptions.push(
      vscode.window.onDidChangeActiveTextEditor((editor) => {
        this._onDidChangeActiveTextEditor(editor)
      }),
    )
    context.subscriptions.push(
      vscode.workspace.onDidChangeTextDocument((event) => {
        this._onDidChangeTextDocument(event)
      }),
    )
    context.subscriptions.push(
      vscode.commands.registerCommand("squawk.showSyntaxTree", async () => {
        const doc = await vscode.workspace.openTextDocument(this._uri)
        await vscode.window.showTextDocument(doc, {
          viewColumn: vscode.ViewColumn.Beside,
          preserveFocus: true,
        })
      }),
    )

    // initial kick off to make sure we have the editor set
    this._onDidChangeActiveTextEditor(vscode.window.activeTextEditor)
  }

  onDidChange = this._eventEmitter.event

  _onDidChangeActiveTextEditor(editor: vscode.TextEditor | undefined) {
    if (editor && isSqlEditor(editor)) {
      this._activeEditor = editor
      this._eventEmitter.fire(this._uri)
    }
  }

  _onDidChangeTextDocument(event: vscode.TextDocumentChangeEvent) {
    if (isSqlDocument(event.document)) {
      // via rust-analzyer:
      // We need to order this after language server updates, but there's no API for that.
      // Hence, good old sleep().
      void sleep(10).then(() => this._eventEmitter.fire(this._uri))
    }
  }

  async provideTextDocumentContent(_uri: vscode.Uri): Promise<string> {
    try {
      const document = this._activeEditor?.document
      if (!document) {
        vscode.window.showErrorMessage("Error: no active editor found")
        return ""
      }
      if (!client) {
        vscode.window.showErrorMessage("Error: no client found")
        return ""
      }
      const uri = document.uri.toString()
      log.info(`Requesting syntax tree for: ${uri}`)
      const response = await client.sendRequest<string>("squawk/syntaxTree", {
        textDocument: { uri },
      })
      log.info("Syntax tree received")
      return response
    } catch (error) {
      log.error("Failed to get syntax tree:", error)
      vscode.window.showErrorMessage(
        `Failed to get syntax tree:\n${String(error)}`,
      )
      return ""
    }
  }
}

class TokensProvider implements vscode.TextDocumentContentProvider {
  _eventEmitter = new vscode.EventEmitter<vscode.Uri>()
  _activeEditor: vscode.TextEditor | undefined
  _uri = vscode.Uri.parse("squawk-tokens://tokens/tokens.rast")

  constructor(context: vscode.ExtensionContext) {
    context.subscriptions.push(
      vscode.window.onDidChangeActiveTextEditor((editor) => {
        this._onDidChangeActiveTextEditor(editor)
      }),
    )
    context.subscriptions.push(
      vscode.workspace.onDidChangeTextDocument((event) => {
        this._onDidChangeTextDocument(event)
      }),
    )
    context.subscriptions.push(
      vscode.commands.registerCommand("squawk.showTokens", async () => {
        const doc = await vscode.workspace.openTextDocument(this._uri)
        await vscode.window.showTextDocument(doc, {
          viewColumn: vscode.ViewColumn.Beside,
          preserveFocus: true,
        })
      }),
    )

    // initial kick off to make sure we have the editor set
    this._onDidChangeActiveTextEditor(vscode.window.activeTextEditor)
  }

  onDidChange = this._eventEmitter.event

  _onDidChangeActiveTextEditor(editor: vscode.TextEditor | undefined) {
    if (editor && isSqlEditor(editor)) {
      this._activeEditor = editor
      this._eventEmitter.fire(this._uri)
    }
  }

  _onDidChangeTextDocument(event: vscode.TextDocumentChangeEvent) {
    if (isSqlDocument(event.document)) {
      // via rust-analzyer:
      // We need to order this after language server updates, but there's no API for that.
      // Hence, good old sleep().
      void sleep(10).then(() => this._eventEmitter.fire(this._uri))
    }
  }

  async provideTextDocumentContent(_uri: vscode.Uri): Promise<string> {
    try {
      const document = this._activeEditor?.document
      if (!document) {
        vscode.window.showErrorMessage("Error: no active editor found")
        return ""
      }
      if (!client) {
        vscode.window.showErrorMessage("Error: no client found")
        return ""
      }
      const uri = document.uri.toString()
      log.info(`Requesting tokens for: ${uri}`)
      const response = await client.sendRequest<string>("squawk/tokens", {
        textDocument: { uri },
      })
      log.info("Tokens received")
      return response
    } catch (error) {
      log.error("Failed to get tokens:", error)
      vscode.window.showErrorMessage(
        `Error: Failed to get tokens:\n${String(error)}`,
      )
      return ""
    }
  }
}

function sleep(ms: number): Promise<void> {
  return new Promise((resolve) => setTimeout(resolve, ms))
}

function assertNever(param: never): never {
  throw new Error(`should never get here, but got ${String(param)}`)
}
