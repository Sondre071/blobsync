#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod utils;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 800.0])
            .with_min_inner_size([600.0, 800.0])
            .with_transparent(true),
        ..Default::default()
    };

    eframe::run_native(
        "blobsync",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<app::types::App>::default())
        }),
    )
}
