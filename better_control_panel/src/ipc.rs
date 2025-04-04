use std::io::{self, BufReader, Read, Write};

use interprocess::local_socket::{GenericFilePath, GenericNamespaced, ListenerOptions, prelude::*};
use serde::{Deserialize, Serialize};
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
    EncodeError(bincode::error::EncodeError),
    DecodeError(bincode::error::DecodeError),
    ConnectError(io::Error),
    WriteError(io::Error),
    ReadError(io::Error),
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
            Self::EncodeError(e) => write!(f, "failed to encode: {}", e),
            Self::DecodeError(e) => write!(f, "failed to decode: {}", e),
            Self::ConnectError(e) => write!(f, "failed to connect: {}", e),
            Self::WriteError(e) => write!(f, "failed to write: {}", e),
            Self::ReadError(e) => write!(f, "failed to read: {}", e),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::SingleInstanceError(e) => Some(e),
            Self::AlreadyRunning => None,
            Self::ToNsNameError(e) => Some(e),
            Self::AddrInUseError(e) => Some(e),
            Self::ListenerCreateError(e) => Some(e),
            Self::SetNonBlockingError(e) => Some(e),
            Self::EncodeError(e) => Some(e),
            Self::DecodeError(e) => Some(e),
            Self::ConnectError(e) => Some(e),
            Self::WriteError(e) => Some(e),
            Self::ReadError(e) => Some(e),
        }
    }
}

pub struct Server {
    _instance: SingleInstance,
    listener: LocalSocketListener,
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
        })
    }
    pub unsafe fn receiver(&self) -> Result<Option<BufReader<LocalSocketStream>>> {
        let binding = self.listener.accept();
        match binding {
            Ok(conn) => Ok(Some(BufReader::new(conn))),
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => return Ok(None),
            Err(e) => Err(e).map_err(Error::ConnectError)?,
        }
    }
    pub fn recevie_str(&self) -> Result<Option<String>> {
        let mut reader = match unsafe { self.receiver() }? {
            Some(reader) => reader,
            None => return Ok(None),
        };
        let mut message = String::new();
        reader
            .read_to_string(&mut message)
            .map_err(Error::ReadError)?;
        Ok(Some(message))
    }
    pub unsafe fn recevie_bytes(&self) -> Result<Option<Vec<u8>>> {
        let mut reader = match unsafe { self.receiver() }? {
            Some(reader) => reader,
            None => return Ok(None),
        };
        let mut message = Vec::new();
        reader.read_to_end(&mut message).map_err(Error::ReadError)?;
        Ok(Some(message))
    }
    pub fn recevie<T>(&self) -> Result<Option<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let mut reader = match unsafe { self.receiver() }? {
            Some(reader) => reader,
            None => return Ok(None),
        };
        let message =
            bincode::serde::decode_from_std_read(&mut reader, bincode::config::standard())
                .map_err(Error::DecodeError)?;
        Ok(Some(message))
    }
}

///
/// sender
///
#[derive(Debug)]
pub struct Sender {
    conn: LocalSocketStream,
}

impl Sender {
    pub fn new(app_id: &str) -> Result<Self> {
        let socket_path = if GenericNamespaced::is_supported() {
            format!("{}.sock", app_id).to_ns_name::<GenericNamespaced>()
        } else {
            format!("/tmp/{}.sock", app_id).to_fs_name::<GenericFilePath>()
        }
        .map_err(Error::ToNsNameError)?;
        let conn = LocalSocketStream::connect(socket_path).map_err(Error::ConnectError)?;
        Ok(Self { conn })
    }
    pub fn send_bytes(&mut self, message: &[u8]) -> Result<()> {
        self.conn.write_all(message).map_err(Error::WriteError)?;
        Ok(())
    }
    pub fn send_str(&mut self, message: &str) -> Result<()> {
        self.conn
            .write_all(message.as_bytes())
            .map_err(Error::WriteError)?;
        Ok(())
    }
    pub fn send<T>(&mut self, message: T) -> Result<()>
    where
        T: Serialize,
    {
        self.send_bytes(
            &bincode::serde::encode_to_vec(message, bincode::config::standard())
                .map_err(Error::EncodeError)?,
        )
    }
}
