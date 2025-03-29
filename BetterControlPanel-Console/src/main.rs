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

impl App {
    fn new(args: Cli) -> Self {
        let app_id: String = better_control_panel::app_id!();
        let server = match better_control_panel::ipc::Server::new(&app_id) {
            Ok(server) => server,
            Err(better_control_panel::ipc::Error::AlreadyRunning) => {
                println!("Another instance is already running. Send args to it...");
                better_control_panel::ipc::send_args_to_server(&app_id).unwrap();
                std::process::exit(0)
            }
            Err(e) => {
                eprintln!("创建 IPC 服务失败：{}", e);
                std::process::exit(0)
            }
        };
        let result = Self { server };
        result.on_message(&args.script);
        result
    }

    fn on_message(&self, message: &str) {
        match better_control_panel::ipc::send_str_to_server(
            &better_control_panel::app_id!("BetterControlPanel-Server"),
            message,
        ) {
            Ok(()) => {
                println!("发送消息给服务端成功：{}", message);
            }
            Err(e) => {
                eprintln!("发送消息给服务端失败：{}", e);
            }
        }
    }

    fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            match self.server.next() {
                Ok(Some(message)) => {
                    println!("收到来自其他程序的消息：{}", message);
                    self.on_message(&message);
                }
                Ok(None) => {}
                Err(e) => {
                    eprintln!("接收来自其他程序的消息失败：{}", e);
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::new(Cli::parse()).run()
}
