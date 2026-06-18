use crate::backend::Backend;
use crate::shared::Shared;
use crate::shared::account::Account;

mod landing_screen;
mod main_screen;
mod runtime;

pub struct App {
    screen: Screen,
    shared: Shared,
}

impl Default for App {
    fn default() -> Self {
        Self {
            screen: Screen::Landing,
            shared: Shared::new(),
        }
    }
}

enum Screen {
    Landing,
    Main(Box<MainState>),
}

struct MainState {
    backend: Backend,

    containers: Vec<String>,
    current_container: Option<String>,
    current_blobs: Option<Vec<String>>,

    displayed_blob: Option<Blob>,
}

impl MainState {
    fn new(account: &Account) -> Self {
        let backend = Backend::connect(account);
        backend.dispatch_fetch_containers_list_message();

        Self {
            backend,
            containers: Vec::<String>::new(),
            current_container: None,
            current_blobs: None,
            displayed_blob: None,
        }
    }
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

struct Blob {
    name: String,
    container: String,
    bytes: Vec<u8>,
}
