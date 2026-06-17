use super::{App, Blob, Message, Screen};

impl eframe::App for App {
    // Runtime loop
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Poll for messages
        if let Some(backend) = &self.backend {
            while let Ok(msg) = backend.receiver.try_recv() {
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
        }

        match self.state.screen {
            Screen::Main => self.render_main_screen(ui),
            Screen::Landing => self.render_landing_screen(ui),
        }
    }
}
