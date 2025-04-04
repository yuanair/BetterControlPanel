#![windows_subsystem = "windows"]

use std::{io::Write, sync::mpsc, thread};

use better_control_panel::{app_id, util::command::ReciverCommand};
use clap::Parser;
use eframe::egui::{self, Color32};
use log::{error, info};

/// Better Control Panel
#[derive(Debug, Parser)]
#[command(name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"))]
struct Cli {}

struct App {
    main_thread_sender: std::sync::mpsc::Sender<ReciverCommand>,
    rhai_script: String,
}

impl App {
    fn new(main_thread_sender: std::sync::mpsc::Sender<ReciverCommand>) -> Self {
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
                match better_control_panel::ipc::Sender::new(&app_id!("BetterControlPanel-Server"))
                {
                    Ok(mut sender) => {
                        match sender.send(better_control_panel::util::command::Command::Exec {
                            app_id: better_control_panel::app_id!(),
                            script: self.rhai_script.clone(),
                        }) {
                            Ok(_) => {
                                info!("脚本发送成功");
                            }
                            Err(e) => {
                                error!("脚本发送失败：{}", e);
                            }
                        }
                    }
                    Err(e) => {
                        error!("创建 IPC 客户端失败：{}", e);
                    }
                };
            }
            ui.separator();
            better_control_panel::eframe::LogPanel::new().show(ui);
        });
    }
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.main_thread_sender.send(ReciverCommand::Exit).unwrap();
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
            better_control_panel::ipc::Sender::new(&app_id)?
                .send(ReciverCommand::Args(std::env::args().collect()))?;
            return Ok(());
        }
        Err(e) => {
            error!("创建 IPC 服务失败：{}", e);
            return Ok(());
        }
    };

    let bcp_server = match std::process::Command::new("BetterControlPanel-Server.exe").spawn() {
        Ok(bcp_server) => bcp_server,
        Err(e) => {
            error!("启动 BetterControlPanel-Server.exe 失败：{}", e);
            return Ok(());
        }
    };

    let (cmd_sender, cmd_receiver) = mpsc::channel::<ReciverCommand>();
    let main_thread = thread::spawn(move || {
        loop {
            if let Ok(cmd) = cmd_receiver.try_recv() {
                match cmd {
                    ReciverCommand::ExecResult { result } => {
                        info!("脚本执行结果：{}", result);
                    }
                    ReciverCommand::Args(args) => match Cli::try_parse_from(args.clone()) {
                        Ok(_args) => {
                            info!("收到来自其他程序的消息：{}", args.join(" "));
                        }
                        Err(e) => {
                            error!("解析命令行参数失败：{}", e);
                        }
                    },
                    ReciverCommand::Exit => return,
                }
            }
            let message = server.recevie::<ReciverCommand>();
            match message {
                Ok(Some(command)) => match command {
                    ReciverCommand::ExecResult { result } => {
                        info!("脚本执行结果：{}", result);
                    }
                    ReciverCommand::Args(args) => match Cli::try_parse_from(args.clone()) {
                        Ok(_args) => {
                            info!("收到来自其他程序的消息：{}", args.join(" "));
                        }
                        Err(e) => {
                            error!("解析命令行参数失败：{}", e);
                        }
                    },
                    ReciverCommand::Exit => return,
                },
                Ok(None) => {}
                Err(e) => {
                    error!("接收消息失败：{}", e);
                }
            }
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
    better_control_panel::ipc::Sender::new(&better_control_panel::app_id!(
        "BetterControlPanel-Server"
    ))?
    .send(better_control_panel::util::command::Command::Exit)?;
    bcp_server.wait_with_output()?;
    Ok(())
}
