use super::types::{CurrentContainer, Location, MainState, Message};

use egui::Ui;
use egui_extras::{Column, TableBuilder};
use std::sync::Arc;

mod polling;

pub fn run_main_screen(ui: &mut Ui, state: &mut MainState) {
    polling::poll_for_messages(state);

    let heading_height = ui.text_style_height(&egui::TextStyle::Heading);
    let row_height = ui.text_style_height(&egui::TextStyle::Body) + 4.0;

    egui::Panel::left("left_side_list")
        .min_size(150.0)
        .resizable(true)
        .show_inside(ui, |ui| {
            ui.add_space(5.0);
            ui.add_sized(
                [ui.available_width(), heading_height],
                egui::Label::new(egui::RichText::new("Containers").heading()),
            );

            ui.separator();

            egui::ScrollArea::vertical()
                .auto_shrink(false)
                .show(ui, |ui| {
                    let clicked =
                        state.containers.iter().find_map(|container| {
                            ui.button(container)
                                .clicked()
                                .then(|| container.clone())
                        });

                    if let Some(container) = clicked {
                        state.switch_to_container(ui, container);
                    }
                });
        });

    if let (Some(blob), Some(container)) =
        (&state.displayed_blob, &state.current_container)
        && let Some(bytes) = &blob.bytes
    {
        let max_width = (ui.available_width() * 0.4).max(0.0);

        let uri = format!("bytes://{}/{}", container.name, blob.name);
        let image = egui::Image::from_bytes(uri, Arc::clone(bytes));

        let desired_size = image
            .load_and_calc_size(
                ui,
                egui::vec2(max_width, ui.available_height()),
            )
            .map(|s| s.x.min(max_width))
            .unwrap_or(max_width);

        egui::Panel::right("preview_panel")
            .resizable(true)
            .default_size(desired_size)
            .size_range(100.0..=max_width.max(100.0))
            .show_inside(ui, |ui| {
                ui.add(image);
            });
    }

    egui::CentralPanel::default().show_inside(ui, |ui| {
        if let Some(container) = &state.current_container {
            ui.add_sized(
                [ui.available_width(), heading_height],
                egui::Label::new(
                    egui::RichText::new(&container.name).heading(),
                ),
            );

            ui.separator();

            TableBuilder::new(ui)
                .column(Column::remainder().at_least(120.0))
                .column(Column::auto().at_least(60.0))
                .column(Column::auto().at_least(60.0))
                .column(Column::auto().at_least(50.0))
                .striped(true)
                .header(heading_height * 0.8, |mut header| {
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
                    body.rows(row_height, container.blobs.len(), |mut row| {
                        let blob = &container.blobs[row.index()];
                        
                        let name: String = blob.name.chars().take(30).collect();
                        
                        let status = match blob.location {
                            Location::Remote => "Remote",
                            Location::Local => "Local",
                            Location::Synced => "Synced",
                        };

                        let length = format!("{} kb", &blob.length / 1024);

                        row.col(|ui| {
                            ui.label(&blob.name);
                        });

                        row.col(|ui| {
                            ui.label(status);
                        });

                        row.col(|ui| {
                            ui.label(length);
                        });

                        row.col(|ui| {
                            if ui.button("View").clicked() {
                                state.backend.dispatch_fetch_blob(
                                    ui.ctx(),
                                    container,
                                    blob,
                                );
                            };
                        });
                    })
                });
        }
    });
}
