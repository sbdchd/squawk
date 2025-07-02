import * as vscode from "vscode"
import { execFileSync } from "child_process"
import {
  LanguageClient,
  LanguageClientOptions,
  Executable,
  ServerOptions,
} from "vscode-languageclient/node"

let client: LanguageClient | undefined
let log: Pick<
  vscode.LogOutputChannel,
  "trace" | "debug" | "info" | "warn" | "error" | "show"
>

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
        vscode.window.showErrorMessage(`Failed to get server version: ${error}`)
      }
    }),
  )

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

  const statusBarItem = vscode.window.createStatusBarItem(
    vscode.StatusBarAlignment.Right,
    100,
  )
  statusBarItem.text = "Squawk"
  statusBarItem.tooltip = "Click to show Squawk Language Server logs"
  statusBarItem.command = "squawk.showLogs"
  statusBarItem.show()
  context.subscriptions.push(statusBarItem)

  await startServer(context)
}

export async function deactivate() {
  await client?.stop()
}

function isSqlDocument(document: vscode.TextDocument): boolean {
  return document.languageId === "sql" || document.languageId === "postgres"
}

function isSqlEditor(editor: vscode.TextEditor): boolean {
  return isSqlDocument(editor.document)
}

function getSquawkPath(context: vscode.ExtensionContext): vscode.Uri {
  const ext = process.platform === "win32" ? ".exe" : ""
  return vscode.Uri.joinPath(context.extensionUri, "server", `squawk${ext}`)
}

async function startServer(context: vscode.ExtensionContext) {
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
    outputChannel: vscode.window.createOutputChannel("Squawk Language Server"),
  }
  client = new LanguageClient(
    "squawk",
    "Squawk Language Server",
    serverOptions,
    clientOptions,
  )

  log.info("Language client created, starting...")
  client.start()
  log.info("Language client started")
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
        this._onDidChangeTextDocument(event.document)
      }),
    )
    context.subscriptions.push(
      vscode.commands.registerCommand("squawk.showSyntaxTree", async () => {
        const doc = await vscode.workspace.openTextDocument(this._uri)
        await vscode.window.showTextDocument(doc, vscode.ViewColumn.Beside)
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

  _onDidChangeTextDocument(document: vscode.TextDocument) {
    if (
      isSqlDocument(document) &&
      this._activeEditor &&
      document === this._activeEditor.document
    ) {
      this._eventEmitter.fire(this._uri)
    }
  }

  async provideTextDocumentContent(_uri: vscode.Uri): Promise<string> {
    try {
      const document = this._activeEditor?.document
      if (!document) {
        return "Error: no active editor found"
      }
      const text = document.getText()
      const uri = document.uri.toString()
      log.info(`Requesting syntax tree for: ${uri}`)
      const response = await client?.sendRequest("squawk/syntaxTree", {
        textDocument: { uri },
        text,
      })
      log.info("Syntax tree received")
      return response as string
    } catch (error) {
      log.error(`Failed to get syntax tree: ${error}`)
      return `Error: Failed to get syntax tree: ${error}`
    }
  }
}
