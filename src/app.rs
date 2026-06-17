use crate::backend::*;

pub struct App {
    state: State,
    backend: Backend,
}

#[derive(Default)]
struct State {
    containers: Vec<String>,

    current_container: Option<String>,
    current_blobs: Option<Vec<String>>,

    displayed_blob: Option<Blob>,
}

struct Blob {
    name: String,
    container: String,
    bytes: Vec<u8>,
}

#[derive(Debug)]
pub enum Message {
    Containers(Vec<String>),
    Blobs {
        container: String,
        blobs: Vec<String>,
    },
    Blob {
        name: String,
        container: String,
        bytes: Vec<u8>,
    },
}

impl Default for App {
    fn default() -> Self {
        let backend = Backend::new();
        backend.list_containers();

        Self {
            state: State::default(),
            backend,
        }
    }
}

impl eframe::App for App {
    // Runtime loop
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Poll for messages
        while let Ok(msg) = self.backend.receiver.try_recv() {
            println!("Handling message: {:?}", &msg);

            match msg {
                Message::Containers(names) => self.state.containers = names,
                Message::Blobs { container, blobs } => {
                    self.state.current_container = Some(container);
                    self.state.current_blobs = Some(blobs);
                }
                Message::Blob {
                    name,
                    container,
                    bytes,
                } => {
                    let blob = Blob {
                        name,
                        container,
                        bytes,
                    };

                    self.state.displayed_blob = Some(blob);
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
                        for container in self.state.containers.iter() {
                            if ui.button(container).clicked() {
                                self.backend.list_blobs(ui, container);
                            }
                        }
                    });
            });

        if let Some(blob) = &self.state.displayed_blob {
            let max_width = (ui.available_width() - 460.0).max(0.0);

            let uri = format!("bytes://{}/{}", blob.container, blob.name);
            let image = egui::Image::from_bytes(uri, blob.bytes.clone());

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
            if let (Some(container), Some(blobs)) =
                (&self.state.current_container, &self.state.current_blobs)
            {
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
                                self.backend.fetch_blob(ui, container, blob);
                            };
                        }
                    });
            }
        });
    }
}
