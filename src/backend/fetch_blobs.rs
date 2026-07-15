use super::{Backend, Message};
use crate::app::{Blob, Location};
use crate::shared;

use egui::Context;
use futures::TryStreamExt;
use std::path::Path;
use std::sync::Arc;
use tokio::fs;
use walkdir::WalkDir;

impl Backend {
    pub(super) fn dispatch_fetch_remote_container(&self, ctx: &Context, container: &str) {
        let sender = self.sender.clone();
        let client = Arc::clone(&self.client);
        let container = container.to_owned();
        let ctx = ctx.clone();

        self.runtime.spawn(async move {
            let mut pager = client
                .blob_container_client(&container)
                .list_blobs(None)
                .unwrap()
                .into_pages();

            let mut blobs = Vec::<Blob>::new();

            while let Some(page) = pager.try_next().await.unwrap() {
                let list = page.into_model().unwrap();

                for item in list.blob_items {
                    let name = item.name.unwrap();

                    let Some(properties) = item.properties else {
                        println!("Blob properties was None.");

                        continue;
                    };

                    let length = properties
                        .content_length
                        .expect("Unable to get content length.");

                    let content_md5: [u8; 16] = properties
                        .content_md5
                        .expect("No md5-hash found for the file.")
                        .try_into()
                        .expect("Failed to parse md5-hash into 16-byte uint.");

                    let blob = Blob::new(name, length, None, content_md5, Location::Remote);
                    blobs.push(blob);
                }
            }

            sender
                .send(Message::Blobs { container, blobs })
                .expect("Failed to fetch remote blobs.");

            ctx.request_repaint();
        });
    }

    pub(super) fn dispatch_fetch_local_blobs(&self, ctx: &Context, container: &str) {
        let local_container_name = container.replace('-', "_");
        let root = Path::new(&self.account.local_path).join(&local_container_name);
        let sender = self.sender.clone();
        let ctx = ctx.clone();

        if root.try_exists().is_err() {
            shared::println!(
                "%tNo local folder found for container: '%n{}%t', returning.",
                local_container_name
            );

            return;
        }

        self.runtime.spawn_blocking(move || {
            let mut blobs = Vec::<Blob>::new();

            for file in WalkDir::new(&root)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|e| e.file_type().is_file())
            {
                // TODO: Stream byte reading and md5 hashing in case of large files.
                let Ok(bytes) = std::fs::read(file.path()) else {
                    let feedback = format!(
                        "Failed to read file: {}, container: {}",
                        file.file_name().to_string_lossy(),
                        &local_container_name
                    );

                    eprintln!("{}", feedback);
                    continue;
                };

                let length = bytes.len() as u64;
                let name = file.file_name().to_string_lossy().to_string();

                let digest = md5::compute(bytes);

                let blob = Blob::new(name, length, None, digest.0, Location::Local);
                blobs.push(blob);
            }

            sender
                .send(Message::Blobs {
                    container: local_container_name,
                    blobs,
                })
                .expect("Failed to fetch local blobs.");

            ctx.request_repaint();
        });
    }

    pub fn dispatch_fetch_blob(&self, ctx: &Context, container: &str, blob: &Blob) {
        let sender = self.sender.clone();
        let client = Arc::clone(&self.client);
        let container = container.to_owned();
        let ctx = ctx.clone();

        let blob = blob.clone();
        let local_root = Path::new(&self.account.local_path)
            .join(&container)
            .join(&blob.name);

        shared::println!("%tFetching blob: %n{}/{}\n", container, blob.name);

        self.runtime.spawn(async move {
            let bytes = {
                if blob.location != Location::Remote {
                    fs::read(local_root)
                        .await
                        .expect("Failed to read local file.")
                } else {
                    let response = client
                        .blob_client(&container, &blob.name)
                        .download(None)
                        .await
                        .unwrap();

                    response
                        .body
                        .collect()
                        .await
                        .expect("Failed to read blob bytes.")
                        .to_vec()
                }
            };

            let blob = Blob::new(blob.name, blob.length, Some(bytes), blob.md5, blob.location);

            sender
                .send(Message::BlobWithBytes(blob))
                .expect("Failed to fetch blob bytes.");

            ctx.request_repaint();
        });
    }
}
