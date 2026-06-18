use super::MainState;
use super::Screen;
use crate::shared::Shared;

use egui::Ui;

pub fn render_landing_screen(ui: &mut Ui, shared: &mut Shared) -> Option<Screen> {
    let mut next: Option<Screen> = None;

    for account in &shared.accounts {
        if ui.button(&account.name).clicked() {
            let main_state = Box::new(MainState::new(account));

            next = Some(Screen::Main(main_state));
        }
    }

    next
}
