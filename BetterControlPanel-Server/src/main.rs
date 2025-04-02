#![windows_subsystem = "windows"]
use std::fs::File;

use clap::Parser;
use log::{error, info};

/// Better Control Panel Server
#[derive(Debug, Parser)]
#[command(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"))]
struct Cli {}

struct App {
    server: better_control_panel::ipc::Server,
    rhai_engine: rhai::Engine,
}

impl App {
    fn new(args: Cli) -> Self {
        let app_id: String = better_control_panel::app_id!();
        let server = match better_control_panel::ipc::Server::new(&app_id) {
            Ok(server) => server,
            Err(better_control_panel::ipc::Error::AlreadyRunning) => {
                info!("Another instance is already running. Send args to it...");
                better_control_panel::ipc::send_args_to_server(&app_id).unwrap();
                std::process::exit(0)
            }
            Err(e) => {
                error!("创建 IPC 服务失败：{}", e);
                std::process::exit(0)
            }
        };
        Self {
            server,
            rhai_engine: rhai::Engine::new(),
        }
    }

    fn on_message(&self, message: &str) {
        let result = self.run_script(message);
        match result {
            Ok(value) => {
                info!("脚本执行结果：{:?}", value);
            }
            Err(e) => {
                error!("脚本执行失败：{}", e);
            }
        }
    }

    fn run_script(&self, script: &str) -> Result<rhai::Dynamic, Box<rhai::EvalAltResult>> {
        self.rhai_engine.eval::<rhai::Dynamic>(&script)
    }

    fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            match self.server.next() {
                Ok(Some(message)) => {
                    info!("收到来自其他程序的消息：{}", message);
                    self.on_message(&message);
                }
                Ok(None) => {}
                Err(e) => {
                    error!("接收来自其他程序的消息失败：{}", e);
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_default_env()
        .target(env_logger::Target::Pipe(Box::new(File::create(
            "Server.log",
        )?)))
        .init();
    App::new(Cli::parse()).run()
}
