#![windows_subsystem = "windows"]
use std::fs::File;

use better_control_panel::{
    ipc::Sender,
    log::pop_global_buffer,
    util::command::{Command, ReciverCommand},
};
use clap::Parser;
use log::{debug, error, info};

/// Better Control Panel Server
#[derive(Debug, Parser)]
#[command(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"))]
struct Cli {}

struct App {
    server: better_control_panel::ipc::Server,
    rhai_engine: rhai::Engine,
}

impl App {
    fn new(_args: Cli) -> Self {
        let app_id: String = better_control_panel::app_id!();
        let server = match better_control_panel::ipc::Server::new(&app_id) {
            Ok(server) => server,
            Err(better_control_panel::ipc::Error::AlreadyRunning) => {
                info!("Another instance is already running. Send args to it...");
                better_control_panel::ipc::Sender::new(&app_id)
                    .unwrap()
                    .send(Command::Args(std::env::args().collect()))
                    .unwrap();
                std::process::exit(0)
            }
            Err(e) => {
                error!("创建 IPC 服务失败：{}", e);
                std::process::exit(0)
            }
        };
        let mut rhai_engine = rhai::Engine::new();
        rhai_engine
            .on_print(|s| info!("Rhai: {}", s))
            .on_debug(|s, src, pos| {
                debug!(
                    "Rhai Debug: {}\n\tat{}: {}",
                    s,
                    pos,
                    src.unwrap_or_default()
                )
            });

        better_control_panel::util::registe_to_rhai(&mut rhai_engine);
        Self {
            server,
            rhai_engine,
        }
    }

    fn run_script(&self, script: &str) -> Result<rhai::Dynamic, Box<rhai::EvalAltResult>> {
        self.rhai_engine.eval::<rhai::Dynamic>(&script)
    }

    fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            match self.server.recevie::<Command>() {
                Ok(Some(command)) => match command {
                    Command::Exec {
                        app_id: sender_app_id,
                        script,
                    } => {
                        debug!("接收到执行脚本命令：{}", script);
                        match self.run_script(&script) {
                            Ok(value) => {
                                Sender::new(&sender_app_id)?.send(ReciverCommand::ExecResult {
                                    result: value.to_string(),
                                })?;
                                info!("脚本执行结果：{:?}", value);
                            }
                            Err(e) => {
                                error!("脚本执行失败：{}", e);
                            }
                        }
                    }
                    Command::Args(args) => match Cli::try_parse_from(&args) {
                        Ok(_) => {
                            debug!("接收到命令行参数：{}", args.join(" "));
                        }
                        Err(e) => {
                            error!("解析命令行参数失败：{}", e);
                        }
                    },
                    Command::Exit => {
                        info!("收到退出命令");
                        break Ok(());
                    }
                },
                Ok(None) => {}
                Err(e) => {
                    error!("接收来自其他程序的消息失败：{}", e);
                }
            }
            if let Some(message) = pop_global_buffer().unwrap() {
                Sender::new("BetterControlPanel")?.send(ReciverCommand::Log {
                    app_id: better_control_panel::app_id!(),
                    message: message.to_buf(),
                })?;
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // better_control_panel::log::init()?;
    better_control_panel::log::redirect_panic_to_log();
    env_logger::Builder::from_default_env()
        .target(env_logger::Target::Pipe(Box::new(File::create(
            "Server.log",
        )?)))
        .init();
    App::new(Cli::parse()).run()
}
