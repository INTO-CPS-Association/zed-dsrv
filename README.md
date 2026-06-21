# DSRV support for Zed

This directory contains a Zed extension for the DSRV language.

## Included features

- File recognition for `*.dsrv`
- Tree-sitter based syntax highlighting
- Language Server Protocol support through `dsrv-lsp`

## Setup

1. Build the language server binary:

   - `cargo build --release --manifest-path server/Cargo.toml`

2. Put `dsrv-lsp` on your `PATH`.

3. In Zed, run **Install Dev Extension** and pick this directory:

   - `zed-extension`

## Notes

- The extension starts the language server by looking for `dsrv-lsp` in the project shell environment.
- If `dsrv-lsp` is not available in `PATH`, Zed will show an error from the extension.
- The grammar is configured as a local `file://` repository in `extension.toml`.
- If you move this project folder, update `[grammars.dsrv].repository` to the new absolute `file://` path of `zed-extension/tree-sitter-dsrv`.
