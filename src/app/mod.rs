use crate::backend::Backend;
use crate::backend::credentials::Account;

mod landing_screen;
mod main_screen;
mod runtime;

#[derive(Default)]
pub struct App {
    state: State,
    backend: Option<Backend>,
}

#[derive(Default)]
struct State {
    screen: Screen,

    accounts: Vec<Account>,

    containers: Vec<String>,
    current_container: Option<String>,
    current_blobs: Option<Vec<String>>,

    displayed_blob: Option<Blob>,
}

#[derive(Default)]
enum Screen {
    #[default]
    Landing,
    Main,
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
