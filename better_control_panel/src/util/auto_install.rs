use std::fmt::{Display, Formatter};

///
/// Error
///
#[derive(Debug)]
pub enum Error {
    /// Unsuported platform
    UnsuportedPlatform,
    /// Command execution error
    CommandExecutionError(std::io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnsuportedPlatform => write!(f, "Unsuported platform"),
            Error::CommandExecutionError(e) => write!(f, "Command execution error: {}", e),
        }
    }
}

impl std::error::Error for Error {}

///
/// Result
///
pub type Result<T> = std::result::Result<T, Error>;

///
/// auto install info
///
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AutoInstallInfo {}

///
/// rust proxy info
///
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RustProxyInfo {
    /// the environment 'RUSTUP_DIST_SERVER'
    dist_server: String,
    /// the environment 'RUSTUP_UPDATE_ROOT'
    update_root: String,
}

///
/// rust auto install info
///
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RustAutoInstallInfo {
    /// proxy
    pub proxy: Option<RustProxyInfo>,
}

///
/// Install Rust
///
pub fn rust(info: RustAutoInstallInfo) -> Result<()> {
    let mut vars = std::collections::HashMap::new();
    if let Some(proxy) = info.proxy {
        vars.insert("RUSTUP_DIST_SERVER", proxy.dist_server);
        vars.insert("RUSTUP_UPDATE_ROOT", proxy.update_root);
    }
    if cfg!(windows) {
        std::process::Command::new("powershell.exe")
            .envs(vars)
            .arg("Invoke-WebRequest")
            .arg("-Uri")
            .arg("https://win.rustup.rs")
            .arg("-OutFile")
            .arg("rustup-init.exe")
            .arg(".\rustup-init.exe")
            .arg("-y")
            .arg("--default-toolchain")
            .arg("stable")
            .spawn()
            .map_err(Error::CommandExecutionError)?
            .wait()
            .map_err(Error::CommandExecutionError)?;
    } else if cfg!(target_os = "linux") {
        std::process::Command::new("curl")
            .envs(vars)
            .arg("--proto")
            .arg("='https'")
            .arg("--tlsv1.2")
            .arg("-sSf")
            .arg("https://sh.rustup.rs")
            .arg("|")
            .arg("sh")
            .arg("-s")
            .arg("--")
            .arg("-y")
            .spawn()
            .map_err(Error::CommandExecutionError)?
            .wait()
            .map_err(Error::CommandExecutionError)?;
    } else {
        return Err(Error::UnsuportedPlatform);
    }
    test_rust()?;
    Ok(())
}

///
/// Test whether Rust is installed to PATH
///
pub fn test_rust() -> Result<()> {
    std::process::Command::new("rustc")
        .arg("--version")
        .spawn()
        .map_err(Error::CommandExecutionError)?
        .wait()
        .map_err(Error::CommandExecutionError)?;
    Ok(())
}
