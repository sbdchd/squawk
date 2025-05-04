import editorWorker from "monaco-editor/esm/vs/editor/editor.worker?worker"

self.MonacoEnvironment = {
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  getWorker(_: unknown, _label: string) {
    return new editorWorker()
  },
}
