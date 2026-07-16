use crate::app::types::Backend;
use crate::utils::{self, Shared, account::Account};

use egui::Context;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

pub struct App {
    pub screen: Screen,
    pub shared: Shared,
}

impl Default for App {
    fn default() -> Self {
        Self {
            screen: Screen::Landing,
            shared: Shared::new(),
        }
    }
}

pub enum Screen {
    Landing,
    Main(Box<MainState>),
}

pub struct MainState {
    pub backend: Backend,

    pub containers: Vec<String>,
    pub current_container: Option<CurrentContainer>,

    pub displayed_blob: Option<Blob>,
}

impl MainState {
    pub fn new(account: &Account) -> Self {
        let backend = Backend::connect(account);

        Self {
            backend,
            containers: Vec::new(),
            current_container: None,
            displayed_blob: None,
        }
    }

    pub fn switch_to_container(
        &mut self,
        ctx: &Context,
        container: impl AsRef<str>,
    ) {
        self.displayed_blob = None;

        self.current_container = Some(CurrentContainer {
            name: container.as_ref().to_owned(),
            blobs: Vec::new(),
        });

        self.backend.dispatch_fetch_remote_container(
            ctx,
            self.current_container.as_ref().unwrap(),
        );
        self.backend.dispatch_fetch_local_blobs(
            ctx,
            self.current_container.as_ref().unwrap(),
        );

        utils::println!(
            "%tCurrent container set to: %n{}\n",
            container.as_ref()
        );
    }
}

pub struct CurrentContainer {
    pub name: String,
    pub blobs: Vec<Blob>,
}

impl CurrentContainer {
    pub fn insert_new_blobs(&mut self, new_blobs: Vec<Blob>) {
        if self.blobs.is_empty() {
            self.blobs = new_blobs;
            return;
        };

        let existing_blob_indices: HashMap<[u8; 16], usize> = self
            .blobs
            .iter()
            .enumerate()
            .map(|(index, blob)| (blob.md5, index))
            .collect();

        new_blobs.into_iter().for_each(|b| {
            if let Some(index) = existing_blob_indices.get(&b.md5) {
                let existing_blob = &mut self.blobs[*index];

                if existing_blob.location != Location::Synced
                    && existing_blob.location != b.location
                {
                    // As there are only three states, the blob HAS to be synced now.
                    existing_blob.location = Location::Synced;
                };
            } else {
                self.blobs.push(b);
            }
        });
    }

    pub fn local_container(
        &self,
        local_account_path: impl AsRef<str>,
    ) -> Option<(String, PathBuf)> {
        let allowed_space_character_replacements = [" ", "-", "_"];

        allowed_space_character_replacements
            .iter()
            .find_map(|char| {
                let path = Path::new(local_account_path.as_ref())
                    .join(self.name.replace('-', char));

                if path.try_exists().unwrap_or(false) {
                    let directory_name =
                        path.file_name().unwrap().to_string_lossy().to_string();

                    Some((directory_name, path))
                } else {
                    None
                }
            })
    }
}

#[derive(Debug)]
pub enum Message {
    Containers(Vec<String>),
    Blobs {
        container: String,
        blobs: Vec<Blob>,
        location: Location,
    },
    BlobWithBytes(Blob),
}

#[derive(Clone, Debug)]
pub struct Blob {
    pub name: String,
    pub length: u64,
    pub bytes: Option<Arc<[u8]>>,
    pub md5: [u8; 16],
    pub location: Location,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Location {
    Local,
    Remote,
    Synced,
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Blob {
    pub fn new(
        name: String,
        length: u64,
        bytes: Option<Vec<u8>>,
        md5: [u8; 16],
        location: Location,
    ) -> Self {
        Self {
            name,
            length,
            bytes: bytes.map(Arc::from),
            md5,
            location,
        }
    }
}
