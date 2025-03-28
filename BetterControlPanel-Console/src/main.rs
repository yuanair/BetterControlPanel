use clap::Parser;
use single_instance::SingleInstance;

/// Better Control Panel Console
#[derive(Debug, Parser)]
#[command(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"))]
struct Cli {
    /// The script to run
    #[arg(short, long)]
    script: String,
}

fn main() {
    let args = Cli::parse();
    println!("{:?}", args);
    let instance =
        SingleInstance::new(env!("CARGO_PKG_NAME")).expect("Failed to create single instance");
    if !instance.is_single() {
        eprintln!("程序已在运行，请勿重复启动！");
        return;
    }
    println!("程序启动成功，单例运行中...");
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
