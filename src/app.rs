use crate::backend::*;

pub struct App {
    state: State,
    backend: Backend,
}

#[derive(Default)]
struct State {
    containers: Vec<String>,

    current_container: String,
    current_blobs: Vec<String>,
}

pub enum Message {
    Containers(Vec<String>),
    Blobs {
        container: String,
        blobs: Vec<String>,
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
            match msg {
                Message::Containers(names) => self.state.containers = names,
                Message::Blobs { container, blobs } => {}
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
                                // fetch list of blobs
                            }
                        }
                    });
            });
    }
}
