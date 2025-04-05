use std::{
    fmt::{Display, Formatter},
    process::Output,
};

#[cfg(feature = "rhai")]
use rhai::{CustomType, TypeBuilder};

use super::process::CommandExt;

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

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::UnsuportedPlatform => None,
            Error::CommandExecutionError(e) => Some(e),
        }
    }
}

#[cfg(feature = "rhai")]
impl From<Error> for Box<rhai::EvalAltResult> {
    fn from(e: Error) -> Self {
        e.to_string().into()
    }
}

///
/// Result
///
pub type Result<T> = std::result::Result<T, Error>;

///
/// Install Result
///
pub type InstallResult = Result<Output>;

///
/// install trait
///
pub trait Install {
    ///
    /// install
    ///
    fn install(self) -> InstallResult;
    ///
    /// test if installed correctly
    ///
    fn test() -> InstallResult;
}

///
/// rust proxy
///
#[cfg_attr(feature = "rhai", derive(CustomType))]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RustProxy {
    /// the environment 'RUSTUP_DIST_SERVER'
    pub dist_server: String,
    /// the environment 'RUSTUP_UPDATE_ROOT'
    pub update_root: String,
}

impl RustProxy {
    pub const RUSTUP_DIST_SERVER: &'static str = "RUSTUP_DIST_SERVER";
    pub const RUSTUP_UPDATE_ROOT: &'static str = "RUSTUP_UPDATE_ROOT";
    pub fn as_env_args(&self) -> std::collections::HashMap<&'static str, &String> {
        let mut map = std::collections::HashMap::new();
        map.insert(Self::RUSTUP_DIST_SERVER, &self.dist_server);
        map.insert(Self::RUSTUP_UPDATE_ROOT, &self.update_root);
        map
    }
    pub fn into_env_args(self) -> std::collections::HashMap<&'static str, String> {
        let mut map = std::collections::HashMap::new();
        map.insert(Self::RUSTUP_DIST_SERVER, self.dist_server);
        map.insert(Self::RUSTUP_UPDATE_ROOT, self.update_root);
        map
    }
}

///
/// rust
///
#[cfg_attr(feature = "rhai", derive(CustomType))]
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Rust {
    /// proxy
    pub proxy: Option<RustProxy>,
}

impl Install for Rust {
    fn install(self) -> InstallResult {
        let vars = self
            .proxy
            .as_ref()
            .map(RustProxy::as_env_args)
            .unwrap_or_default();
        let output = if cfg!(windows) {
            std::process::Command::new("powershell.exe")
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .stdin(std::process::Stdio::null())
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
                .wait_with_output()
                .map_err(Error::CommandExecutionError)?
        } else if cfg!(target_os = "linux") {
            std::process::Command::new("curl")
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .stdin(std::process::Stdio::null())
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
                .wait_with_output()
                .map_err(Error::CommandExecutionError)?
        } else {
            return Err(Error::UnsuportedPlatform);
        };
        Ok(output)
    }
    fn test() -> InstallResult {
        Ok(std::process::Command::new("rustc")
            .arg("--version")
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .stdin(std::process::Stdio::null())
            .spawn_without_window()
            .map_err(Error::CommandExecutionError)?
            .wait_with_output()
            .map_err(Error::CommandExecutionError)?)
    }
}

#[cfg(feature = "rhai")]
pub fn registe_to_rhai(engine: &mut rhai::Engine) {
    use rhai::EvalAltResult;

    engine.build_type::<RustProxy>();
    engine.build_type::<Rust>();
    engine.register_fn("test", || -> std::result::Result<(), Box<EvalAltResult>> {
        let output = Rust::test()?;

        if !output.stdout.is_empty() {
            log::info!("rustc output: {}", String::from_utf8_lossy(&output.stdout));
        }
        if !output.stderr.is_empty() {
            log::error!("rustc error: {}", String::from_utf8_lossy(&output.stderr));
        }
        if !output.status.success() {
            Err(Error::CommandExecutionError(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "rustc command failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                ),
            )))?
        }
        Ok(())
    });
    // let mut rust_module = Module::new();
    // rust_module.set_native_fn("test", Rust::test);

    // engine.register_static_module("Rust", rust_module.into());
}
