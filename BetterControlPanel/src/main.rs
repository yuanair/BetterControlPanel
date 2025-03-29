use std::{
    io::{self, Write},
    sync::mpsc,
    thread,
};

use better_control_panel::eframe::log_panel;
use clap::Parser;
use eframe::egui::{self, Color32};
use log::{error, info};

/// Better Control Panel
#[derive(Debug, Parser)]
#[command(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"))]
struct Cli {}

enum Command {
    Exit,
}

struct App {
    main_thread_sender: std::sync::mpsc::Sender<Command>,
}

impl App {
    fn new(main_thread_sender: std::sync::mpsc::Sender<Command>) -> Self {
        Self { main_thread_sender }
    }
}

impl eframe::App for App {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        Color32::TRANSPARENT.to_normalized_gamma_f32()
    }
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals {
            window_fill: egui::Color32::MAGENTA,
            panel_fill: egui::Color32::TRANSPARENT,
            ..Default::default()
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            log_panel(ui);
        });
    }
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.main_thread_sender.send(Command::Exit).unwrap();
    }
}
fn on_message(message: io::Result<Option<String>>) {
    match message {
        Ok(Some(message)) => {
            let args: Cli = match Cli::try_parse_from(message.split_ascii_whitespace()) {
                Ok(args) => args,
                Err(e) => {
                    error!("解析命令行参数失败：{}", e);
                    return;
                }
            };
            info!("收到来自其他程序的消息：{:?}", args);
        }
        Ok(None) => {}
        Err(e) if e.kind() == io::ErrorKind::WouldBlock => {}
        Err(e) => {
            error!("接收消息失败：{}", e);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_default_env()
        .target(env_logger::Target::Stdout)
        .format(|buf, record| {
            let log_message = better_control_panel::log::LogMessage::new(record);
            buf.write_fmt(format_args!("{}", log_message))?;
            better_control_panel::log::write_global_buffer(log_message);
            Ok(())
        })
        .init();
    let _args = Cli::parse();
    let (cmd_sender, cmd_receiver) = mpsc::channel::<Command>();
    let server = better_control_panel::ipc::Server::new(better_control_panel::app_id!())?;
    let main_thread = thread::spawn(move || {
        loop {
            if let Ok(cmd) = cmd_receiver.try_recv() {
                match cmd {
                    Command::Exit => return,
                }
            }
            let message = server.next();
            on_message(message);
        }
    });
    better_control_panel::eframe::Builder::new(
        "Better Control Panel".to_owned(),
        Box::new(App::new(cmd_sender)),
    )
    .with_icon(Some(
        image::load_from_memory(include_bytes!("../rc/icons/logo.png"))?.to_rgba8(),
    ))
    .build()?;
    main_thread.join().unwrap();
    Ok(())
}
