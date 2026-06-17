mod shared;

use eframe;
use egui;

fn main() -> eframe::Result {
    eframe::run_native("blobsync",
        Default::default(),
        Box::new(|_|
            Ok(Box::<App>::default())
        )
    )
}

#[derive(Default)]
struct App {
    screen: usize,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {}

    fn ui(&mut self, ctx: &mut egui::Ui, _: &mut eframe::Frame) {}
}
