#![windows_subsystem = "windows"]

use std::{
    io::{self, Write},
    sync::mpsc,
    thread,
};

use better_control_panel::app_id;
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
    rhai_script: String,
}

impl App {
    fn new(main_thread_sender: std::sync::mpsc::Sender<Command>) -> Self {
        Self {
            main_thread_sender,
            rhai_script: "".to_string(),
        }
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
            ui.heading("Better Control Panel");
            let mut is_running = ui.text_edit_singleline(&mut self.rhai_script).lost_focus();
            // if let Some(result) = &*self.rhai_result.read().unwrap() {
            //     ui.label(match result {
            //         Ok(v) => RichText::new(v).color(Color32::GREEN),
            //         Err(e) => RichText::new(e).color(Color32::RED),
            //     });
            // }
            is_running |= ui.button("Run").clicked();
            if is_running {
                match better_control_panel::ipc::send_str_to_server(
                    &app_id!("BetterControlPanel-Server"),
                    &self.rhai_script,
                ) {
                    Ok(()) => {
                        info!("发送脚本给服务端成功");
                    }
                    Err(e) => {
                        error!("发送脚本给服务端失败：{}", e);
                    }
                };
            }
            ui.separator();
            better_control_panel::eframe::LogPanel::new().show(ui);
        });
    }
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.main_thread_sender.send(Command::Exit).unwrap();
    }
}
fn on_message(message: io::Result<Option<String>>) {
    match message {
        Ok(Some(message)) => match Cli::try_parse_from(message.split_ascii_whitespace()) {
            Ok(_args) => {
                info!("收到来自其他程序的消息：{}", message);
            }
            Err(e) => {
                error!("解析命令行参数失败：{}", e);
            }
        },
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
            buf.write_fmt(format_args!("{}\n", log_message))?;
            better_control_panel::log::write_global_buffer(log_message);
            Ok(())
        })
        .init();
    let _args = Cli::parse();
    let app_id: String = better_control_panel::app_id!();
    let server = match better_control_panel::ipc::Server::new(&app_id) {
        Ok(server) => server,
        Err(better_control_panel::ipc::Error::AlreadyRunning) => {
            println!("Another instance is already running. Send args to it...");
            better_control_panel::ipc::send_args_to_server(&app_id)?;
            return Ok(());
        }
        Err(e) => {
            error!("创建 IPC 服务失败：{}", e);
            return Ok(());
        }
    };

    let (cmd_sender, cmd_receiver) = mpsc::channel::<Command>();
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
        image::load_from_memory(include_bytes!("../rc/icons/logo.ico"))?.to_rgba8(),
    ))
    .build()?;
    main_thread.join().unwrap();
    Ok(())
}
