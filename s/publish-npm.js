#!/usr/bin/env node
"use strict"

const childProcess = require("child_process")
const fs = require("fs")
const path = require("path")

/** @type {readonly string[]} */
const PLATFORMS = [
  "darwin-x64",
  "darwin-arm64",
  "linux-x64",
  "linux-arm64",
  "linux-x64-musl",
  "linux-arm64-musl",
  "win32-x64",
]

/**
 * @param {string} name
 * @param {string} version
 * @returns {boolean}
 */
function isPublished(name, version) {
  const result = childProcess.spawnSync(
    "npm",
    ["view", `${name}@${version}`, "version"],
    { encoding: "utf8" },
  )
  return result.status === 0 && result.stdout.trim() === version
}

/**
 * @param {string} dir
 * @returns {void}
 */
function publish(dir) {
  /** @type {{ name: string, version: string }} */
  const { name, version } = JSON.parse(
    fs.readFileSync(path.join(dir, "package.json"), "utf8"),
  )
  if (isPublished(name, version)) {
    console.log(`skipping ${name}@${version}, already published`)
    return
  }
  childProcess.execFileSync(
    "npm",
    ["publish", "--access", "public", "--provenance"],
    { cwd: dir, stdio: "inherit" },
  )
}

/** @returns {void} */
function main() {
  for (const platform of PLATFORMS) {
    publish(path.join("npm", platform))
  }
  publish(".")
}

main()
