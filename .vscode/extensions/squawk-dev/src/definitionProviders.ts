import * as vscode from "vscode"
import * as path from "path"
import { LocationLink, Position, Range } from "vscode"

export class TestSnapshotDefinitionProvider
  implements vscode.DefinitionProvider
{
  public async provideDefinition(
    document: vscode.TextDocument,
    position: vscode.Position,
    token: vscode.CancellationToken,
  ): Promise<vscode.Definition | vscode.LocationLink[] | null> {
    // crates/parser/src/snapshots/parser__alter_table_test__parse_alter_column.snap
    const currentFilePath = document.uri.fsPath

    // crates/parser/src/snapshots
    const snapshotDir = path.dirname(currentFilePath)
    // crates/parser/src
    const srcDir = path.dirname(snapshotDir)
    // crates/parser
    const parserDir = path.dirname(srcDir)
    // crates
    const cratesDir = path.dirname(parserDir)
    // workspaceRoot
    const workspaceRoot = path.dirname(cratesDir)

    const header = document.getText(
      new Range(new Position(0, 0), new Position(3, 0)),
    )
    let inputFilePathRel: string | null = null
    for (const line of header.split("\n")) {
      if (line.startsWith("input_file")) {
        inputFilePathRel = line.slice("input_file: ".length)
      }
    }

    if (inputFilePathRel == null) {
      await vscode.window.showErrorMessage("input file path not found")
      return null
    }
    const inputFilePath = path.join(workspaceRoot, inputFilePathRel)
    const absolutePath = vscode.Uri.file(inputFilePath)

    return testFuncLocation(absolutePath, position, document)
  }
}

function getCursorPosition(
  document: vscode.TextDocument,
  position: Position,
): [number | null, number | null] {
  const cursorLine = document.lineAt(position.line).text

  const matches = cursorLine.match(/^\s*\S+@(\d+\.*\d*)/m)
  // 10..20
  // 10
  const digits = matches?.[1]

  /*
    starts are:

    ^\s*r#"

    ends are:
    ^\s*"#

    */

  let start: number | null = null
  let end: number | null = null
  if (digits != null) {
    if (digits.includes("..")) {
      const [startChars, endChars] = digits.split("..")
      start = parseInt(startChars, 10)
      end = parseInt(endChars, 10)
    } else {
      start = parseInt(digits, 10)
      end = null
    }
  }
  return [start, end]
}

async function testFuncLocation(
  testFilePath: vscode.Uri,
  cursorPosition: Position,
  document: vscode.TextDocument,
): Promise<vscode.Location | vscode.LocationLink[] | null> {
  const testFileDoc = await vscode.workspace.openTextDocument(testFilePath)

  const [byteOffsetStart, byteOffsetEnd] = getCursorPosition(
    document,
    cursorPosition,
  )

  const destFunctionPositionStart = testFileDoc.positionAt(byteOffsetStart!)

  let endCharacterPos = destFunctionPositionStart.character
  if (byteOffsetEnd != null && byteOffsetStart != null) {
    endCharacterPos += byteOffsetEnd - byteOffsetStart!
  }

  const destFunctionPositionEnd = destFunctionPositionStart.with({
    character: endCharacterPos,
  })

  const cursorLine = document.lineAt(cursorPosition.line).text
  let leadingWhiteSpaceEnd = 0
  for (; leadingWhiteSpaceEnd < cursorLine.length; leadingWhiteSpaceEnd++) {
    if (cursorLine[leadingWhiteSpaceEnd] === " ") {
      continue
    } else {
      break
    }
  }

  const originSelectionRange = new Range(
    new Position(cursorPosition.line, leadingWhiteSpaceEnd),
    new Position(cursorPosition.line, cursorLine.length),
  )

  return [
    {
      originSelectionRange,
      targetUri: testFilePath,
      targetRange: new Range(
        destFunctionPositionStart,
        destFunctionPositionEnd,
      ),
      targetSelectionRange: new Range(
        destFunctionPositionStart,
        destFunctionPositionEnd,
      ),
    } satisfies LocationLink,
  ]
}
