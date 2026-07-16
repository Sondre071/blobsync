use crate::app::types::{MainState, Screen};
use crate::shared::Shared;

use egui::Ui;

pub fn render(ui: &mut Ui, shared: &mut Shared) -> Option<Screen> {
    let mut next: Option<Screen> = None;

    for account in &shared.accounts {
        if ui.button(&account.name).clicked() {
            let main_state = Box::new(MainState::new(account));
            main_state.backend.dispatch_fetch_remote_containers_list();

            next = Some(Screen::Main(main_state));
        }
    }

    next
}
