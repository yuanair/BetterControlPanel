use better_control_panel::util::command::ReciverCommand;
use clap::Parser;

/// Better Control Panel Console
#[derive(Debug, Parser)]
#[command(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"))]
struct Cli {
    /// The script to run
    #[arg(short, long, default_value = "")]
    script: String,
}

struct App {
    server: better_control_panel::ipc::Server,
}

impl Drop for App {
    fn drop(&mut self) {
        match better_control_panel::ipc::Sender::new(&better_control_panel::app_id!(
            "BetterControlPanel-Server"
        )) {
            Ok(mut sender) => {
                match sender.send(better_control_panel::util::command::Command::Exit) {
                    Ok(()) => {}
                    Err(e) => {
                        eprintln!("发送消息给服务端失败：{}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("发送消息给服务端失败：{}", e);
            }
        }
    }
}

impl App {
    fn new(args: Cli) -> Self {
        let app_id: String = better_control_panel::app_id!();
        let server = match better_control_panel::ipc::Server::new(&app_id) {
            Ok(server) => server,
            Err(better_control_panel::ipc::Error::AlreadyRunning) => {
                println!("Another instance is already running. Send args to it...");
                better_control_panel::ipc::Sender::new(&app_id)
                    .unwrap()
                    .send(ReciverCommand::Args(std::env::args().collect()))
                    .unwrap();
                std::process::exit(0)
            }
            Err(e) => {
                eprintln!("创建 IPC 服务失败：{}", e);
                std::process::exit(0)
            }
        };
        let result = Self { server };
        result.run_script(&args.script);
        result
    }

    fn run_script(&self, script: &str) {
        match better_control_panel::ipc::Sender::new(&better_control_panel::app_id!(
            "BetterControlPanel-Server"
        )) {
            Ok(mut sender) => {
                match sender.send(better_control_panel::util::command::Command::Exec {
                    app_id: better_control_panel::app_id!(),
                    script: script.to_string(),
                }) {
                    Ok(()) => {
                        println!("发送消息给服务端成功：{}", script);
                    }
                    Err(e) => {
                        eprintln!("发送消息给服务端失败：{}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("发送消息给服务端失败：{}", e);
            }
        }
    }

    fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let bcp_server = match std::process::Command::new("BetterControlPanel-Server.exe").spawn() {
            Ok(bcp_server) => bcp_server,
            Err(e) => {
                eprintln!("启动服务端失败：{}", e);
                return Ok(());
            }
        };
        loop {
            match self.server.recevie::<ReciverCommand>() {
                Ok(Some(command)) => match command {
                    ReciverCommand::Log { app_id, message } => {
                        println!("{}: {}", app_id, message);
                    }
                    ReciverCommand::ExecResult { result } => {
                        println!("执行结果：{}", result);
                        break;
                    }
                    ReciverCommand::Args(args) => match Cli::try_parse_from(&args) {
                        Ok(cli) => {
                            self.run_script(&cli.script);
                        }
                        Err(e) => {
                            eprintln!("解析命令行参数失败：{}", e);
                        }
                    },
                    ReciverCommand::Exit => {
                        println!("退出...");
                        break;
                    }
                },
                Ok(None) => {}
                Err(e) => {
                    eprintln!("接收来自其他程序的消息失败：{}", e);
                }
            }
        }
        better_control_panel::ipc::Sender::new(&better_control_panel::app_id!(
            "BetterControlPanel-Server"
        ))?
        .send(better_control_panel::util::command::Command::Exit)?;
        bcp_server.wait_with_output()?;

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::new(Cli::parse()).run()
}
