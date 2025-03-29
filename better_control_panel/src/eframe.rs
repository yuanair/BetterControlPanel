use eframe::egui::{self, Color32, Grid, text::LayoutJob};
use raw_window_handle::HasWindowHandle;

pub struct Builder {
    app_name: String,
    options: eframe::NativeOptions,
    app: Box<dyn eframe::App>,
}

impl Builder {
    pub fn new(app_name: String, app: Box<dyn eframe::App>) -> Self {
        Self {
            app_name,
            options: eframe::NativeOptions {
                viewport: egui::ViewportBuilder::default(),
                window_builder: Some(Box::new(|builder| {
                    #[cfg(any(target_os = "windows", target_os = "macos"))]
                    builder.with_transparent(true)
                })),
                ..Default::default()
            },
            app,
        }
    }
    pub fn with_option(mut self, option: eframe::NativeOptions) -> Self {
        self.options = option;
        self
    }
    pub fn build(self) -> eframe::Result<()> {
        eframe::run_native(
            &self.app_name,
            self.options,
            Box::new(|cc| {
                egui_extras::install_image_loaders(&cc.egui_ctx);
                let window = cc.window_handle().unwrap();
                #[cfg(target_os = "windows")]
                window_vibrancy::apply_mica(window, None).unwrap();
                #[cfg(target_os = "macos")]
                window_vibrancy::apply_vibrancy(
                    &window,
                    window_vibrancy::NSVisualEffectMaterial::WindowBackground,
                    None,
                    None,
                )
                .unwrap();
                let mut fonts = egui::FontDefinitions::default();
                fonts.font_data.insert(
                    "MSYH".to_owned(),
                    egui::FontData::from_static(include_bytes!("../rc/fonts/MSYH.TTC")).into(),
                );
                fonts
                    .families
                    .get_mut(&egui::FontFamily::Proportional)
                    .unwrap()
                    .insert(0, "MSYH".to_owned());
                fonts
                    .families
                    .get_mut(&egui::FontFamily::Monospace)
                    .unwrap()
                    .push("MSYH".to_owned());

                cc.egui_ctx.set_fonts(fonts);
                Ok(self.app)
            }),
        )
    }
}

pub fn log_panel(ui: &mut egui::Ui) {
    let logs = crate::log::read_global_buffer().unwrap();

    Grid::new("debug_grid").striped(true).show(ui, |ui| {
        // 表头
        ui.style_mut().override_text_style = Some(egui::TextStyle::Heading);
        const HEADERS: &[&str] = &["Time", ("Thread"), ("Level"), ("Module Path"), ("Message")];
        for (col_idx, spec) in HEADERS.iter().enumerate() {
            //ui.set_width(self.max_widths[col_idx]);
            let mut job = LayoutJob::default();
            job.append(
                &spec,
                0.0,
                egui::TextFormat::simple(
                    egui::TextStyle::Heading.resolve(ui.style()),
                    Color32::WHITE,
                ),
            );
            ui.label(job);
        }
        ui.end_row();
    });
    egui::ScrollArea::vertical()
        .stick_to_bottom(true)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label("time");
                ui.label("thread");
                ui.label("level");
                ui.label("module_path");
                ui.label("message");
            });
            ui.separator();
            for log_message in logs.iter() {
                let color = match log_message.level {
                    log::Level::Trace => egui::Color32::MAGENTA,
                    log::Level::Debug => egui::Color32::BLUE,
                    log::Level::Info => egui::Color32::WHITE,
                    log::Level::Warn => egui::Color32::YELLOW,
                    log::Level::Error => egui::Color32::RED,
                };

                ui.horizontal(|ui| {
                    ui.colored_label(color, format!("{}", log_message.local_time));
                    ui.colored_label(
                        color,
                        format!("{}", log_message.thread.name().unwrap_or_default()),
                    );
                    ui.colored_label(color, format!("{}", log_message.level));
                    ui.colored_label(
                        color,
                        format!("{}", log_message.module_path.as_deref().unwrap_or_default()),
                    );
                    ui.colored_label(color, format!("{}", log_message.message));
                });
            }
        });
}
