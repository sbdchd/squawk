# squawk-vscode

> Visual Studio Code support for Squawk

Surface SQL related lint errors directly in VSCode.

## Install

### From online marketplace

Open the [online marketplace listing](https://marketplace.visualstudio.com/items?itemName=sbdchd.squawk) for Squawk and click "Install". Follow the prompts to open VSCode and install Squawk.

### From VSCode

In VSCode, type `CMD`+`P`, run `Extensions: Install Extensions`, search for `sbdchd.squawk` and click install.

### From Github release

Download the extension package from the [latest Github release](https://github.com/sbdchd/squawk/releases/latest) and run `code --install-extension squawk-*.vsix`

### From source

With `vsce` installed from NPM (`npm install -g @vscode/vsce`), clone [this repo](https://github.com/sbdchd/vscode-squawk) and run `vsce package`. Install the resulting package with `code --install-extension squawk-*.vsix`

## Settings

```json
{
  // enable tracing logs
  "squawk.trace.server": "verbose"
}
```

## Dev

Make sure you're on a vscode version >= the one defined in the `package.json`,
otherwise the extension development host won't load the extension.
