#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod backend;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 800.0])
            .with_min_inner_size([1000.0, 800.0])
            .with_transparent(true),
        ..Default::default()
    };

    eframe::run_native(
        "blobsync",
        options,
        Box::new(|_| Ok(Box::<app::App>::default())),
    )
}
