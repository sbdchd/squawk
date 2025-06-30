import * as vscode from "vscode"
import { execFileSync } from "child_process"
import {
  LanguageClient,
  LanguageClientOptions,
  Executable,
  ServerOptions,
} from "vscode-languageclient/node"

let client: LanguageClient | undefined

export async function activate(context: vscode.ExtensionContext) {
  console.log("Squawk activate")

  const serverVersionCommand = vscode.commands.registerCommand(
    "squawk.serverVersion",
    () => {
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
    },
  )
  context.subscriptions.push(serverVersionCommand)

  await startServer(context)
}

export async function deactivate() {
  await client?.stop()
}

function getSquawkPath(context: vscode.ExtensionContext): vscode.Uri {
  const ext = process.platform === "win32" ? ".exe" : ""
  return vscode.Uri.joinPath(context.extensionUri, "server", `squawk${ext}`)
}

async function startServer(context: vscode.ExtensionContext) {
  console.log("starting squawk server")

  const squawkPath = getSquawkPath(context)
  const hasBinary = await vscode.workspace.fs.stat(squawkPath).then(
    () => true,
    () => false,
  )
  if (!hasBinary) {
    const errorMsg = `Squawk binary not found at: ${squawkPath.path}`
    console.error(errorMsg)
    vscode.window.showErrorMessage(errorMsg)
    return
  }
  console.log(`Found Squawk binary at: ${squawkPath}`)

  const serverExecutable: Executable = {
    command: squawkPath.path,
    args: ["server", "--verbose"],
  }
  const serverOptions: ServerOptions = serverExecutable
  const clientOptions: LanguageClientOptions = {
    documentSelector: [
      { scheme: "file", language: "sql" },
      { scheme: "file", language: "postgres" },
    ],
    outputChannel: vscode.window.createOutputChannel("Squawk Language Server"),
  }
  client = new LanguageClient(
    "squawk",
    "Squawk Language Server",
    serverOptions,
    clientOptions,
  )

  client.start()
}
