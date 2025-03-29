use std::sync::Arc;

use eframe::egui::{self, Color32, Grid, IconData, ScrollArea, text::LayoutJob};
use raw_window_handle::HasWindowHandle;

pub struct Builder {
    app_name: String,
    options: eframe::NativeOptions,
    app: Box<dyn eframe::App>,
}

impl Builder {
    pub fn new(app_name: String, app: Box<dyn eframe::App>) -> Self {
        // let icon_data = include_bytes!("../rc/icons/logo.png");
        // let img = image::load_from_memory_with_format(icon_data, image::ImageFormat::Png).unwrap();
        // let rgba_data = img.into_rgba8();
        // let (w, h) = (rgba_data.width(), rgba_data.height());
        // let raw_data: Vec<u8> = rgba_data.into_raw();
        // native_options.viewport.icon = Some(Arc::<IconData>::new(IconData {
        //     rgba: raw_data,
        //     width: w,
        //     height: h,
        // }));
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
    pub fn with_icon(mut self, icon: Option<image::RgbaImage>) -> Self {
        self.options.viewport.icon = icon.map(|img| {
            let (width, height) = (img.width(), img.height());
            let rgba: Vec<u8> = img.into_raw();
            Arc::new(IconData {
                rgba,
                width,
                height,
            })
        });
        self
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

    ScrollArea::both().show(ui, |ui| {
        Grid::new("debug_grid").striped(true).show(ui, |ui| {
            ui.style_mut().override_text_style = Some(egui::TextStyle::Heading);
            const HEADERS: &[&str] = &[
                "Time",
                "Thread",
                "Thread ID",
                "Level",
                "Module Path",
                "Message",
            ];
            for (_col_idx, spec) in HEADERS.iter().enumerate() {
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
            ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
            for log_message in logs.iter() {
                let color = match log_message.level {
                    log::Level::Trace => egui::Color32::MAGENTA,
                    log::Level::Debug => egui::Color32::BLUE,
                    log::Level::Info => egui::Color32::WHITE,
                    log::Level::Warn => egui::Color32::YELLOW,
                    log::Level::Error => egui::Color32::RED,
                };
                let raw = [
                    &format!("{}", log_message.local_time),
                    log_message.thread.name().unwrap_or_default(),
                    &format!("{}", log_message.thread.id().as_u64()),
                    &format!("{}", log_message.level),
                    log_message.module_path.as_deref().unwrap_or_default(),
                    &log_message.message,
                ];
                for (_col_idx, cell) in raw.iter().enumerate() {
                    let mut job = LayoutJob::default();
                    job.append(
                        cell,
                        0.0,
                        egui::TextFormat::simple(egui::TextStyle::Body.resolve(ui.style()), color),
                    );

                    ui.label(job);
                }
                ui.end_row();
            }
        });
    });
}
