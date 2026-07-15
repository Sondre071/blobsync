use super::{CurrentContainer, Location, MainState, Message};
use crate::shared;

use egui_extras::{Column, TableBuilder};
use std::sync::Arc;

use egui::Ui;

pub fn render_main_screen(ui: &mut Ui, state: &mut MainState) {
    // Poll for messages
    while let Ok(msg) = state.backend.receiver.try_recv() {
        match msg {
            Message::Containers(containers) => {
                shared::println!("%mMessage received: %nContainers");

                state.containers = containers;
            }
            Message::Blobs { container, blobs } => {
                shared::println!("%mMessage received: %nBlobs");
                shared::println!(
                    "%tContainer: %n{}%t, file count: %d{}",
                    container,
                    blobs.len()
                );

                if let Some(current_container) = &mut state.current_container
                    && current_container.name == container
                {
                    current_container.insert_new_blobs(blobs);
                } else {
                    state.current_container = Some(CurrentContainer {
                        name: container,
                        blobs,
                    });
                }
            }
            Message::BlobWithBytes(blob) => {
                shared::println!("%mMessage received: %nBlobBytes");
                shared::println!("%tFile: %n{}", blob.name);

                state.displayed_blob = Some(blob);
            }
        }

        println!();
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
                            state.backend.switch_to_container(ui.ctx(), container);
                        }
                    }
                });
        });

    if let (Some(blob), Some(container)) = (&state.displayed_blob, &state.current_container)
        && let Some(bytes) = &blob.bytes
    {
        let max_width = (ui.available_width() - 460.0).max(0.0);

        let uri = format!("bytes://{}/{}", container.name, blob.name);
        let image = egui::Image::from_bytes(uri, Arc::clone(bytes));

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
        if let Some(container) = &state.current_container {
            ui.add_sized(
                [200.0, 25.0],
                egui::Label::new(egui::RichText::new(&container.name).heading()),
            );

            ui.separator();

            TableBuilder::new(ui)
                .column(Column::initial(240.0))
                .column(Column::initial(60.0))
                .column(Column::initial(70.0))
                .column(Column::auto())
                .striped(true)
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.label("Name");
                    });

                    header.col(|ui| {
                        ui.label("Status");
                    });

                    header.col(|ui| {
                        ui.label("Size");
                    });
                })
                .body(|body| {
                    body.rows(16.0, container.blobs.len(), |mut row| {
                        let blob = &container.blobs[row.index()];
                        let name = &blob.name[..blob.name.len().min(30)];
                        let status = match blob.location {
                            Location::Remote => "Remote",
                            Location::Local => "Local",
                            Location::Synced => "Synced",
                        };

                        let length = format!("{} kb", &blob.length / 1024);

                        row.col(|ui| {
                            ui.label(name);
                        });

                        row.col(|ui| {
                            ui.label(status);
                        });

                        row.col(|ui| {
                            ui.label(length);
                        });

                        row.col(|ui| {
                            if ui.button("View").clicked() {
                                state
                                    .backend
                                    .dispatch_fetch_blob(ui.ctx(), &container.name, blob);
                            };
                        });
                    })
                });

            if false {
                egui::ScrollArea::vertical()
                    .auto_shrink(false)
                    .show(ui, |ui| {
                        egui::Grid::new("blob_table")
                            .striped(true)
                            .spacing(egui::Vec2::new(10.0, 8.0))
                            .show(ui, |ui| {
                                ui.end_row();
                            });
                    });
            }
        }
    });
}
