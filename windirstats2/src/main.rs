mod app;
mod scanner;
mod treemap;
mod file_item;
mod extension_stats;
mod ui;

use eframe::egui;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon.png")[..])
                    .unwrap_or_default(),
            ),
        ..Default::default()
    };

    eframe::run_native(
        "WinDirStats2 - Disk Usage Analyzer",
        native_options,
        Box::new(|cc| Ok(Box::new(app::WinDirStatsApp::new(cc)))),
    )
}
