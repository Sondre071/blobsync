use crate::backend::*;

pub struct App {
    state: State,
    backend: Backend,
}

#[derive(Default)]
struct State {
    containers: Vec<String>,
}

pub enum Message {
    Containers(Vec<String>),
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
    fn ui(&mut self, ctx: &mut egui::Ui, _: &mut eframe::Frame) {
        // Runtime loop
        egui::CentralPanel::default().show_inside(ctx, |ui| {
            // Check incoming data
            while let Ok(msg) = self.backend.receiver.try_recv() {
                match msg {
                    Message::Containers(names) => self.state.containers = names,
                }
            }

            // Render UI

            if self.state.containers.is_empty() {
                if ui.button("Fetch containers").clicked() {
                    self.backend.list_containers();
                }
            } else {
                for container in self.state.containers.iter() {
                    if ui.button(container).clicked() {
                        //self.backend.
                    }
                }
            }
            ui.label("No containers");
        });
    }
}
