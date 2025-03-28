use std::{
    f32::consts::E,
    io::{self, BufRead, BufReader, Read, Write},
    sync::mpsc,
    thread::{self, JoinHandle},
};

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
}

pub struct Server {
    _instance: SingleInstance,
    listener: LocalSocketListener,
    conn: Option<LocalSocketStream>,
}

impl Server {
    pub fn new(app_id: String) -> Result<Self, Box<dyn std::error::Error>> {
        let instance = single_instance::SingleInstance::new(&app_id)?;
        if !instance.is_single() {
            println!("Another instance is already running. Send args to it...");
            send_args_to_server(&app_id)?;
            std::process::exit(0);
        }

        let printname = format!("{}.sock", app_id);
        let name = printname.clone().to_ns_name::<GenericNamespaced>()?;

        let opts = ListenerOptions::new().name(name);
        let listener = match opts.create_sync() {
            Err(e) if e.kind() == io::ErrorKind::AddrInUse => {
                eprintln!(
                    "Error: could not start server because the socket file is occupied. Please check
                    if {printname} is in use by another process and try again."
                );
                Err(e)?
            }
            x => x?,
        };
        listener.set_nonblocking(interprocess::local_socket::ListenerNonblockingMode::Both)?;

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
                    Err(e) => return Err(e),
                }
            }
        };
        let mut reader = BufReader::new(conn);
        let mut message = String::new();
        if let Err(e) = reader.read_to_string(&mut message) {
            return Err(e);
        }
        Ok(Some(message))
    }
}

/// send args to server
pub fn send_args_to_server(app_id: &str) -> io::Result<()> {
    let message = std::env::args().collect::<Vec<String>>().join(" ");

    let socket_path = if GenericNamespaced::is_supported() {
        format!("{}.sock", app_id).to_ns_name::<GenericNamespaced>()?
    } else {
        format!("/tmp/{}.sock", app_id).to_fs_name::<GenericFilePath>()?
    };
    let mut conn = LocalSocketStream::connect(socket_path)?;

    conn.write_all(message.as_bytes())?;
    Ok(())
}
