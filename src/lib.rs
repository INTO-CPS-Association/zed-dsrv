use zed_extension_api as zed;

const LANGUAGE_SERVER_REPOSITORY: &str = "INTO-CPS-Association/dsrv-lsp";
const LANGUAGE_SERVER_ASSET: &str = "dsrv-lsp";

struct DsrvExtension {
    downloaded_server: Option<String>,
}

impl zed::Extension for DsrvExtension {
    fn new() -> Self {
        Self {
            downloaded_server: None,
        }
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        if let Some(path) = worktree.which(LANGUAGE_SERVER_ASSET) {
            return Ok(zed::Command::new(path));
        }

        if let Some(path) = self.downloaded_server.clone() {
            return Ok(zed::Command::new(path));
        }

        let (os, architecture) = zed::current_platform();
        if !matches!(
            (os, architecture),
            (zed::Os::Linux, zed::Architecture::X8664)
        ) {
            return Err("Could not find `dsrv-lsp` in PATH. Automatic download is currently available only for Linux x86_64; install the language server (`cargo install --git https://github.com/INTO-CPS-Association/dsrv-lsp.git --locked`) or build it from source, then add the resulting binary to your PATH.".to_string());
        }

        let release = zed::latest_github_release(
            LANGUAGE_SERVER_REPOSITORY,
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == LANGUAGE_SERVER_ASSET)
            .ok_or_else(|| {
                format!(
                    "The latest `{LANGUAGE_SERVER_REPOSITORY}` release does not contain a `{LANGUAGE_SERVER_ASSET}` asset."
                )
            })?;
        let binary_path = format!("{LANGUAGE_SERVER_ASSET}-{}", release.version);

        zed::download_file(
            &asset.download_url,
            &binary_path,
            zed::DownloadedFileType::Uncompressed,
        )?;
        zed::make_file_executable(&binary_path)?;

        self.downloaded_server = Some(binary_path.clone());
        Ok(zed::Command::new(binary_path))
    }
}

zed::register_extension!(DsrvExtension);
