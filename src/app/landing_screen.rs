use super::App;
use crate::backend::Backend;
use crate::backend::credentials;

use egui::Ui;

impl App {
    pub fn render_landing_screen(&mut self, ui: &mut Ui) {
        self.state.accounts = credentials::get_storage_accounts().expect("No credentials found.");

        for account in &self.state.accounts {
            if ui.button(&account.name).clicked() {
                let backend = Backend::new(account);
                backend.list_containers();

                self.backend = Some(backend);
                self.state.screen = super::Screen::Main;
            }
        }
    }
}
