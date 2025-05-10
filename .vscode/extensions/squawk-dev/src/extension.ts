import * as vscode from "vscode"
import * as path from "path"
import { TestSnapshotDefinitionProvider } from "./definitionProviders"

function computeSnapshotPath(sqlPath: string): string | undefined {
  // Convert a path like:
  // crates/parser/src/test_data/ok/alter_foreign_table.sql
  // to:
  // crates/parser/src/snapshots/parser__test__alter_foreign_table.snap

  // go from:
  //   crates/parser/test_data/ok/alter_foreign_table.sql
  // to:
  //   crates/parser
  const sourceRoot = path.dirname(path.dirname(path.dirname(sqlPath)))
  const ok_or_err = path.parse(path.dirname(sqlPath)).name
  // alter_foreign_table
  const testName = path.parse(sqlPath).name
  // crates/parser/src/snapshots/parser__test__alter_foreign_table.snap
  return path.join(
    sourceRoot,
    `src/snapshots/parser__test__${testName}_${ok_or_err}.snap`
  )
}

// Code lens provider to jump to snapshot files
class JumpToSnapshotCodeLensProvider implements vscode.CodeLensProvider {
  public provideCodeLenses(
    document: vscode.TextDocument,
    token: vscode.CancellationToken
  ): vscode.CodeLens[] | Thenable<vscode.CodeLens[]> {
    const position = new vscode.Position(0, 0)
    const range = new vscode.Range(position, position)
    const codeLens = new vscode.CodeLens(range, {
      title: "Jump to Snapshot",
      command: "squawk-dev.jumpToSnapshot",
      arguments: [document.uri.fsPath],
    })
    return [codeLens]
  }
}

export function activate(context: vscode.ExtensionContext) {
  console.log('"squawk-dev" active!')

  context.subscriptions.push(
    vscode.languages.registerDefinitionProvider(
      [
        {
          scheme: "file",
          // e.g., crates/parser/src/snapshots/parser__alter_table_test__parse_alter_column.snap
          pattern: "**/snapshots/*.snap",
        },
      ],
      new TestSnapshotDefinitionProvider()
    )
  )

  context.subscriptions.push(
    vscode.languages.registerCodeLensProvider(
      {
        scheme: "file",
        // crates/parser/test_data/ok/alter_foreign_table.sql
        // pattern: "**/test_data/**/*.sql",
        // pattern: "*.sql",
      },
      new JumpToSnapshotCodeLensProvider()
    )
  )

  context.subscriptions.push(
    vscode.commands.registerCommand(
      "squawk-dev.jumpToSnapshot",
      (path: string) => {
        openSnapshotPath(path)
      }
    )
  )
}

async function openSnapshotPath(sqlPath: string) {
  const snapshot = computeSnapshotPath(sqlPath)
  if (!snapshot) {
    vscode.window.showErrorMessage(`couldn't find snapshot path for ${sqlPath}`)
    return
  }
  const doc = await vscode.workspace.openTextDocument(snapshot)
  await vscode.window.showTextDocument(doc)
}

export function deactivate() {}
