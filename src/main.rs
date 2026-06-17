#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod shared;

use std::sync::mpsc::{Receiver, Sender, channel};

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 800.0])
            .with_min_inner_size([1000.0, 800.0])
            .with_transparent(true),
        ..Default::default()
    };

    eframe::run_native("blobsync", options, Box::new(|_| Ok(Box::<App>::default())))
}

struct App {
    state: State,
    backend: Backend,
}

#[derive(Default)]
struct State {
    containers: Vec<String>,
}

struct Backend {
    runtime: tokio::runtime::Runtime,
    sender: Sender<Message>,
    receiver: Receiver<Message>,
}

enum Message {
    Containers(Vec<String>),
}

impl Default for App {
    fn default() -> Self {
        let (sender, receiver) = channel();

        Self {
            state: State::default(),
            backend: Backend {
                runtime: tokio::runtime::Runtime::new().unwrap(),
                sender,
                receiver,
            },
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
                    let sender = self.backend.sender.clone();

                    self.backend.runtime.spawn(async move {
                        let names = shared::list_containers().await.unwrap_or_default();
                        sender
                            .send(Message::Containers(names))
                            .expect("Failed to container data from thread.");
                    });
                }
            } else {
                for container in self.state.containers.iter() {
                    ui.label(container);
                }
            }
            ui.label("No containers");
        });
    }
}
