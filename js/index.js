"use strict"

const childProcess = require("child_process")
const fs = require("fs")

/** @type {Record<string, string>} */
const PLATFORM_PACKAGES = {
  "darwin-x64": "@squawk-cli/darwin-x64",
  "darwin-arm64": "@squawk-cli/darwin-arm64",
  "linux-x64": "@squawk-cli/linux-x64",
  "linux-arm64": "@squawk-cli/linux-arm64",
  "linux-x64-musl": "@squawk-cli/linux-x64-musl",
  "linux-arm64-musl": "@squawk-cli/linux-arm64-musl",
  "win32-x64": "@squawk-cli/win32-x64",
}

function isMusl() {
  // same order as detect-libc, process.report can be slow
  try {
    return fs.readFileSync("/usr/bin/ldd", "utf8").includes("musl")
  } catch {
    /** @type {any} */
    const report = process.report?.getReport()
    return !report?.header.glibcVersionRuntime
  }
}

function getBinaryPath() {
  let key = `${process.platform}-${process.arch}`
  if (process.platform === "linux" && isMusl()) {
    key += "-musl"
  }
  const pkg = PLATFORM_PACKAGES[key]
  if (!pkg) {
    throw new Error(
      `squawk: unsupported platform "${key}". Supported: ${Object.keys(PLATFORM_PACKAGES).join(", ")}.`,
    )
  }
  const binaryName = process.platform === "win32" ? "squawk.exe" : "squawk"
  try {
    return require.resolve(`${pkg}/bin/${binaryName}`)
  } catch {
    throw new Error(
      `squawk: the "${pkg}" optional dependency was not installed.\n` +
        `This usually means your package manager skipped optional dependencies. ` +
        `Re-install with optional dependencies enabled (e.g. "npm install --include=optional").`,
    )
  }
}

function resolveBinaryOrExit() {
  try {
    return getBinaryPath()
  } catch (err) {
    console.error(err instanceof Error ? err.message : err)
    process.exit(1)
  }
}

function run() {
  const binaryPath = resolveBinaryOrExit()

  const child = childProcess.spawn(binaryPath, process.argv.slice(2), {
    stdio: "inherit",
  })

  child.on("error", (err) => {
    console.error("error: failed to invoke squawk")
    console.error(err.stack)
    process.exit(1)
  })

  child.on("exit", (code) => {
    process.exit(code ?? 1)
  })

  process.on("SIGTERM", () => {
    child.kill("SIGTERM")
  })

  process.on("SIGINT", () => {
    child.kill("SIGINT")
  })
}

module.exports = { getBinaryPath, run }
