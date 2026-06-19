use super::{Blob, MainState, Message};

use std::sync::Arc;

use egui::Ui;

pub fn render_main_screen(ui: &mut Ui, state: &mut MainState) {
    // Poll for messages
    while let Ok(msg) = state.backend.receiver.try_recv() {
        println!("Handling message: {:?}", &msg);

        match msg {
            Message::Containers(names) => state.containers = names,
            Message::Blobs { container, blobs } => {
                if state
                    .current_container
                    .as_ref()
                    .is_some_and(|c| c.as_str() == container)
                {
                    state.current_blobs = Some(blobs);
                }
            }
            Message::Blob {
                name,
                container,
                bytes,
            } => {
                state.displayed_blob = Some(Blob::new(name, container, bytes));
            }
            Message::HashedFile {
                name,
                container,
                digest,
            } => {
                println!("Received hash for file: {}/{}", container, name);
                state
                    .hashes
                    .insert(format!("{}/{}", container, name), digest.0);
            }
        }
    }

    egui::Panel::left("left_side_list")
        .min_size(150.0)
        .show_inside(ui, |ui| {
            ui.add_space(5.0);
            ui.add_sized(
                [ui.available_width(), 25.0],
                egui::Label::new(egui::RichText::new("Containers").heading()),
            );

            ui.separator();

            egui::ScrollArea::vertical()
                .auto_shrink(false)
                .show(ui, |ui| {
                    for container in state.containers.iter() {
                        if ui.button(container).clicked() {
                            state.backend.fetch_blobs_list(ui.ctx(), container);
                            state.current_container = Some(container.clone());
                            state
                                .backend
                                .dispatch_local_files_indexing(ui.ctx(), container);
                        }
                    }
                });
        });

    if let Some(blob) = &state.displayed_blob {
        let max_width = (ui.available_width() - 460.0).max(0.0);

        let uri = format!("bytes://{}/{}", blob.container, blob.name);
        let image = egui::Image::from_bytes(uri, Arc::clone(&blob.bytes));

        let desired_size = image
            .load_and_calc_size(ui, egui::vec2(max_width, ui.available_height()))
            .map(|s| s.x.min(max_width))
            .unwrap_or(max_width);

        egui::Panel::right("preview_panel")
            .exact_size(desired_size)
            .show_inside(ui, |ui| {
                ui.add(image);
            });
    }

    egui::CentralPanel::default().show_inside(ui, |ui| {
        if let (Some(container), Some(blobs)) = (&state.current_container, &state.current_blobs) {
            ui.add_sized(
                [200.0, 25.0],
                egui::Label::new(egui::RichText::new(container).heading()),
            );

            ui.separator();

            egui::ScrollArea::vertical()
                .auto_shrink(false)
                .show(ui, |ui| {
                    for blob in blobs {
                        if ui.button(blob).clicked() {
                            state.backend.fetch_blob(ui.ctx(), container, blob);
                        };
                    }
                });
        }
    });
}
