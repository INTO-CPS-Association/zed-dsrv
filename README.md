# DSRV support for Zed

This directory contains a Zed extension for the DSRV language.

## Included features

- File recognition for `*.dsrv`
- Tree-sitter based syntax highlighting
- Language Server Protocol support through `dsrv-lsp`

## Setup

1. Install `dsrv-lsp`:

   - On Linux x86_64, the extension downloads the latest stable release automatically when no `dsrv-lsp` is found on `PATH`.
   - Otherwise, install it with `cargo install --git https://github.com/INTO-CPS-Association/dsrv-lsp.git --locked` or build `dsrv-lsp` from source, then put it on your `PATH`.

2. In Zed, run **Install Dev Extension** and pick this directory:

   - this `zed-dsrv` repository

## Notes

- The extension prefers `dsrv-lsp` from the project shell environment and otherwise downloads the current Linux x86_64 release asset from `INTO-CPS-Association/dsrv-lsp`.
- The current LSP starts directly over stdin/stdout and therefore receives no command-line arguments.
- The grammar is pinned to a `tree-sitter-dsrv` commit in `extension.toml`. It follows the checker syntax as of checker commit 8c71d20d; newer checker constructs need a coordinated grammar update and pin change.

## License

This project is licensed under the INTO-CPS Association Public License (ICAPL). The selected usage mode is documented in `ICA-USAGE-MODE.txt`. See `LICENSE.md`.
