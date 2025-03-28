use clap::Parser;

/// Better Control Panel Console
#[derive(Debug, Parser)]
#[command(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"))]
struct Cli {
    /// The script to run
    #[arg(short, long)]
    script: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    println!("{:?}", args);
    let server = better_control_panel::ipc::Server::new(better_control_panel::app_id!())?;
    loop {
        match server.next() {
            Ok(Some(message)) => match Cli::try_parse_from(message.split_ascii_whitespace()) {
                Ok(args) => {
                    println!("收到来自其他程序的消息：{:?}", args);
                }
                Err(e) => {
                    eprintln!("解析命令行参数失败：{}", e);
                }
            },
            Ok(None) => {}
            Err(e) => {
                eprintln!("接收来自其他程序的消息失败：{}", e);
            }
        }
    }
}
