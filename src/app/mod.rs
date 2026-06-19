use crate::backend::Backend;
use crate::shared::Shared;
use crate::shared::account::Account;

use std::collections::HashMap;
use std::sync::Arc;

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

    hashes: HashMap<String, [u8; 16]>,
}

impl MainState {
    fn new(account: &Account) -> Self {
        let backend = Backend::connect(account);
        backend.dispatch_fetch_containers_list_message();

        Self {
            backend,
            containers: Vec::new(),
            current_container: None,
            current_blobs: None,
            displayed_blob: None,
            hashes: HashMap::new(),
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
    HashedFile {
        name: String,
        container: Arc<str>,
        digest: md5::Digest,
    },
}

struct Blob {
    name: String,
    container: String,
    pub bytes: Arc<[u8]>,
    pub md5: Option<[u8; 16]>,
}

impl Blob {
    pub fn new(name: String, container: String, bytes: Vec<u8>) -> Self {
        Self {
            name,
            container,
            bytes: bytes.into(),
            md5: None,
        }
    }
}
