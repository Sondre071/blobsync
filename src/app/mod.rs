use crate::backend::Backend;
use crate::shared::Shared;
use crate::shared::account::Account;

use std::collections::{HashMap, HashSet};
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
    current_container: Option<CurrentContainer>,

    displayed_blob: Option<Blob>,

    blob_hashes: HashMap<String, HashSet<[u8; 16]>>,
}

struct CurrentContainer {
    name: String,
    blobs: Vec<Blob>
}

impl MainState {
    fn new(account: &Account) -> Self {
        let backend = Backend::connect(account);

        Self {
            backend,
            containers: Vec::new(),
            current_container: None,
            displayed_blob: None,
            blob_hashes: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub enum Message {
    Containers(Vec<String>),
    Blobs {
        container: String,
        blobs: Vec<Blob>,
    },
    BlobBytes {
        name: String,
        length: u64,
        bytes: Vec<u8>,
        md5: [u8; 16],
    },
    HashedFile {
        name: String,
        digest: md5::Digest,
    },
}

#[derive(Debug)]
pub struct Blob {
    name: String,
    length: u64,
    pub bytes: Option<Arc<[u8]>>,
    pub md5: [u8; 16],
}

impl Blob {
    pub fn new(name: String, length: u64, bytes: Option<Vec<u8>>, md5: [u8; 16]) -> Self {
        Self {
            name,
            length,
            bytes: bytes.map(Arc::from),
            md5,
        }
    }
}
