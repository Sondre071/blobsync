use super::types::{App, Screen};
use super::{landing_screen, main_screen};

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        match &mut self.screen {
            Screen::Main(state) => {
                main_screen::poll_for_messages(state);
                main_screen::render(ui, state);
            }
            Screen::Landing => {
                let next_screen = landing_screen::render(ui, &mut self.shared);

                if let Some(next_screen) = next_screen {
                    self.screen = next_screen;
                }
            }
        }
    }
}
