use zed_extension_api as zed;

struct DsrvExtension;

impl zed::Extension for DsrvExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        if let Some(path) = worktree.which("dsrv-lsp") {
            return Ok(zed::Command {
                command: path,
                args: vec![],
                env: Default::default(),
            });
        }

        Err("Could not find `dsrv-lsp` in PATH. Build the language server (`cargo build --release --manifest-path server/Cargo.toml`) and add the resulting binary to your PATH.".to_string())
    }
}

zed::register_extension!(DsrvExtension);
