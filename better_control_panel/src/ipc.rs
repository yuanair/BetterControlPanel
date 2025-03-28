use std::{
    io::{Read, Write},
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
/// startup
pub fn startup(
    app_id: String,
) -> Result<
    (
        mpsc::Receiver<String>,
        JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>>,
        SingleInstance,
    ),
    Box<dyn std::error::Error>,
> {
    let instance = single_instance::SingleInstance::new(&app_id)?;
    if !instance.is_single() {
        send_args_to_server(&app_id)?;
        std::process::exit(0);
    }
    let (tx, rx) = mpsc::channel::<String>();

    let socket = thread::spawn(|| ipc_server(app_id, tx));
    Ok((rx, socket, instance))
}

/// send args to server
pub fn send_args_to_server(app_id: &str) -> Result<(), Box<dyn std::error::Error>> {
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

fn ipc_server(
    app_id: String,
    tx: mpsc::Sender<String>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let printname = format!("{}.sock", app_id);
    let name = printname.to_ns_name::<GenericNamespaced>()?;

    // Configure our listener...
    let opts = ListenerOptions::new().name(name);

    let listener = LocalSocketListener::from_options(opts)?;

    loop {
        let mut conn = match listener.accept() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("接受连接失败: {}", e);
                continue;
            }
        };

        let mut buf = String::new();
        match conn.read_to_string(&mut buf) {
            Ok(_) => {
                if let Err(e) = tx.send(buf) {
                    eprintln!("发送参数失败: {}", e);
                }
            }
            Err(e) => eprintln!("读取数据失败: {}", e),
        }
    }
}
