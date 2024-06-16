#!/usr/bin/env node

"use strict"

// Copyright (c) 2016 Sentry (https://sentry.io/) and individual contributors.
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
//     1. Redistributions of source code must retain the above copyright notice,
// this list of conditions and the following disclaimer.
//
//     2. Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
//
//     3. Neither the name of the Sentry nor the names of its contributors may be
// used to endorse or promote products derived from this software without specific
// prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
// ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// from: https://github.com/getsentry/sentry-cli/blob/acc8d00eb1965d8f49a9f337b7776c782d253ac4/scripts/install.js

const os = require("os")
const fs = require("fs")
const zlib = require("zlib")
const stream = require("stream")
const process = require("process")
const pkgInfo = require("../package.json")
const fetch = require("node-fetch").default
const path = require("path")
const crypto = require("crypto")
const { binaryPath } = require("./helpers")

// e.g.: https://github.com/sbdchd/squawk/releases/download/v0.1.3/squawk-darwin-x86_64
const RELEASES_BASE_URL = "https://github.com/sbdchd/squawk/releases/download"

const SUPPORTED_PLATFORMS = new Set([
  'x86_64-apple-darwin',
  'aarch64-apple-darwin',
  'x86_64-unknown-linux-musl',
  'aarch64-unknown-linux-musl',
])

/**
 * @param {string} platform
 * @param {string} arch
 */
function getDownloadUrl(platform, arch) {
  if (!SUPPORTED_PLATFORMS.has(`${platform}-${arch}`)) {
    return null
  }
  return `${RELEASES_BASE_URL}/v${pkgInfo.version}/squawk-${platform}-${arch}`
}

function getNpmCache() {
  const env = process.env
  return (
    env.npm_config_cache ||
    env.npm_config_yarn_offline_mirror ||
    path.join(os.homedir(), ".npm")
  )
}

/** @param {string} url */
function getCachedPath(url) {
  const digest = crypto.createHash("md5").update(url).digest("hex").slice(0, 6)

  return path.join(
    getNpmCache(),
    "squawk-cli",
    `${digest}-${path.basename(url).replace(/[^a-zA-Z0-9.]+/g, "-")}`
  )
}

/** @param {string} cached */
function getTempFile(cached) {
  return `${cached}.${process.pid}-${Math.random().toString(16).slice(2)}.tmp`
}

/** @param {import("node-fetch").Response} response */
function getDecompressor(response) {
  const contentEncoding = response.headers.get("content-encoding")
  if (contentEncoding == null) {
    return new stream.PassThrough()
  }
  if (/\bgzip\b/.test(contentEncoding)) {
    return zlib.createGunzip()
  }
  if (/\bdeflate\b/.test(contentEncoding)) {
    return zlib.createInflate()
  }
  if (/\bbr\b/.test(contentEncoding)) {
    return zlib.createBrotliDecompress()
  }
  return new stream.PassThrough()
}

function downloadBinary() {
  const arch = os.arch()
  const platform = os.platform()
  const downloadUrl = getDownloadUrl(platform, arch)
  if (!downloadUrl) {
    return Promise.reject(new Error(`unsupported target ${platform}-${arch}`))
  }

  const cachedPath = getCachedPath(downloadUrl)
  if (fs.existsSync(cachedPath)) {
    fs.copyFileSync(cachedPath, binaryPath)
    return Promise.resolve()
  }

  return fetch(downloadUrl, {
    compress: false,
    headers: {
      "accept-encoding": "gzip, deflate, br",
    },
    redirect: "follow",
  }).then(response => {
    if (!response.ok) {
      throw new Error(`Received ${response.status}: ${response.statusText}`)
    }

    const decompressor = getDecompressor(response)

    const tempPath = getTempFile(cachedPath)

    fs.mkdirSync(path.dirname(tempPath), { recursive: true })

    return new Promise((resolve, reject) => {
      response.body
        .on("error", e => reject(e))
        .pipe(decompressor)
        .pipe(fs.createWriteStream(tempPath, { mode: 0o755 }))
        .on("error", e => reject(e))
        .on("close", () => resolve())
    }).then(() => {
      fs.copyFileSync(tempPath, cachedPath)
      fs.copyFileSync(tempPath, binaryPath)
      fs.unlinkSync(tempPath)
    })
  })
}

downloadBinary()
  .then(() => process.exit(0))
  .catch(e => {
    console.error(e)
    process.exit(1)
  })
