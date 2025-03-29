use std::io::{self, BufReader, Read, Write};

use interprocess::local_socket::{GenericFilePath, GenericNamespaced, ListenerOptions, prelude::*};
use single_instance::SingleInstance;

#[macro_export]
/// user name
macro_rules! user_name {
    () => {
        std::env::var(if cfg!(windows) { "USERNAME" } else { "USER" }).unwrap()
    };
}

#[macro_export]
/// application unique id
macro_rules! app_id {
    () => {
        format!(
            "{}_{}_{}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION"),
            $crate::user_name!()
        )
    };
    ($name:expr) => {
        format!(
            "{}_{}_{}",
            $name,
            env!("CARGO_PKG_VERSION"),
            $crate::user_name!()
        )
    };
}

#[derive(Debug)]
pub enum Error {
    SingleInstanceError(single_instance::error::SingleInstanceError),
    AlreadyRunning,
    ToNsNameError(io::Error),
    AddrInUseError(io::Error),
    ListenerCreateError(io::Error),
    SetNonBlockingError(io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<single_instance::error::SingleInstanceError> for Error {
    fn from(e: single_instance::error::SingleInstanceError) -> Self {
        Self::SingleInstanceError(e)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::SingleInstanceError(e) => write!(f, "failed to new SingleInstance: {}", e),
            Self::AlreadyRunning => write!(f, "the application is already running"),
            Self::ToNsNameError(e) => write!(f, "failed to convert to ns name: {}", e),
            Self::AddrInUseError(e) => write!(f, "address in use error: {}", e),
            Self::ListenerCreateError(e) => write!(f, "failed to create listener: {}", e),
            Self::SetNonBlockingError(e) => write!(f, "failed to set non blocking: {}", e),
        }
    }
}

impl std::error::Error for Error {}

pub struct Server {
    _instance: SingleInstance,
    listener: LocalSocketListener,
    conn: Option<LocalSocketStream>,
}

impl Server {
    pub fn new(app_id: &str) -> Result<Self> {
        let instance = single_instance::SingleInstance::new(app_id)?;
        if !instance.is_single() {
            Err(Error::AlreadyRunning)?
        }

        let printname = format!("{}.sock", app_id);
        let name = printname
            .clone()
            .to_ns_name::<GenericNamespaced>()
            .map_err(Error::ToNsNameError)?;

        let opts = ListenerOptions::new().name(name);
        let listener = match opts.create_sync() {
            Err(e) if e.kind() == io::ErrorKind::AddrInUse => {
                Err(e).map_err(Error::AddrInUseError)?
            }
            x => x.map_err(Error::ListenerCreateError)?,
        };
        listener
            .set_nonblocking(interprocess::local_socket::ListenerNonblockingMode::Both)
            .map_err(Error::SetNonBlockingError)?;

        Ok(Self {
            _instance: instance,
            listener,
            conn: None,
        })
    }
    pub fn next(&self) -> io::Result<Option<String>> {
        let binding;
        let conn = match self.conn.as_ref() {
            Some(conn) => conn,
            None => {
                binding = self.listener.accept();
                match binding {
                    Ok(ref conn) => conn,
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => return Ok(None),
                    Err(e) => Err(e)?,
                }
            }
        };
        let mut reader = BufReader::new(conn);
        let mut message = String::new();
        reader.read_to_string(&mut message)?;
        Ok(Some(message))
    }
}

/// send str to server
pub fn send_str_to_server(app_id: &str, message: &str) -> io::Result<()> {
    let socket_path = if GenericNamespaced::is_supported() {
        format!("{}.sock", app_id).to_ns_name::<GenericNamespaced>()?
    } else {
        format!("/tmp/{}.sock", app_id).to_fs_name::<GenericFilePath>()?
    };
    let mut conn = LocalSocketStream::connect(socket_path)?;

    conn.write_all(message.as_bytes())?;
    Ok(())
}

/// send args to server
pub fn send_args_to_server(app_id: &str) -> io::Result<()> {
    send_str_to_server(app_id, &std::env::args().collect::<Vec<String>>().join(" "))
}
