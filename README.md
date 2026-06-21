# DSRV support for Zed

This directory contains a Zed extension for the DSRV language.

## Included features

- File recognition for `*.dsrv`
- Tree-sitter based syntax highlighting
- Language Server Protocol support through `dsrv-lsp`

## Setup

1. Build the language server binary:

   - `cargo install --git https://github.com/INTO-CPS-Association/dsrv-lsp.git --locked` or build `dsrv-lsp` from source

2. Put `dsrv-lsp` on your `PATH`.

3. In Zed, run **Install Dev Extension** and pick this directory:

   - this `zed-dsrv` repository

## Notes

- The extension starts the language server by looking for `dsrv-lsp` in the project shell environment.
- If `dsrv-lsp` is not available in `PATH`, Zed will show an error from the extension.
- The grammar is configured to use `https://github.com/INTO-CPS-Association/tree-sitter-dsrv.git` in `extension.toml`.

## License

This project is licensed under the INTO-CPS Association Public License (ICAPL). The selected usage mode is documented in `ICA-USAGE-MODE.txt`. See `LICENSE.md`.
