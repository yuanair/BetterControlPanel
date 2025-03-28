use clap::Parser;

/// Better Control Panel Console
#[derive(Debug, Parser)]
#[command(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"))]
struct Cli {
    /// The script to run
    #[arg(short, long)]
    script: Option<String>,
}

fn main() {
    let args = Cli::parse();
    println!("{:?}", args);
    let (tx, _join_handle, _instance) =
        better_control_panel::ipc::startup(better_control_panel::app_id!()).unwrap();
    println!("程序启动成功，单例运行中...");
    for message in tx {
        let args = match Cli::try_parse_from(message.split_ascii_whitespace()) {
            Ok(args) => args,
            Err(e) => {
                eprintln!("解析命令行参数失败：{}", e);
                continue;
            }
        };
        println!("收到来自其他程序的消息：{:?}", args);
    }
}
