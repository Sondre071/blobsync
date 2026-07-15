use crate::app::types::{Blob, CurrentContainer, Location};
use crate::backend::{Backend, Message};
use crate::shared;

use egui::Context;
use futures::TryStreamExt;
use std::path::Path;
use std::sync::Arc;
use tokio::fs;
use walkdir::DirEntry;
use walkdir::WalkDir;

impl Backend {
    pub fn dispatch_fetch_remote_container(
        &self,
        ctx: &Context,
        container: &CurrentContainer,
    ) {
        let sender = self.sender.clone();
        let client = Arc::clone(&self.client);
        let container = container.name.to_owned();
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

                    let blob = Blob::new(
                        name,
                        length,
                        None,
                        content_md5,
                        Location::Remote,
                    );
                    blobs.push(blob);
                }
            }

            sender
                .send(Message::Blobs {
                    container,
                    blobs,
                    location: Location::Remote,
                })
                .expect("Failed to fetch remote blobs.");

            ctx.request_repaint();
        });
    }

    pub fn dispatch_fetch_local_blobs(
        &self,
        ctx: &Context,
        container: &CurrentContainer,
    ) {
        const SIZE_THRESHOLD: u64 = 10 * 1024 * 1024;

        let remote_container_name = container.name.to_owned();

        let Some((local_container_name, local_container_path)) =
            container.local_container(&self.account.local_path)
        else {
            shared::println!(
                "%wNo local directory found for container: %n{}%t, returning.",
                &container.name
            );

            return;
        };

        let sender = self.sender.clone();
        let ctx = ctx.clone();

        self.runtime.spawn_blocking(move || {
            let mut small_files: Vec<DirEntry> = Vec::new();
            let mut large_files: Vec<DirEntry> = Vec::new();

            for file in WalkDir::new(&local_container_path)
                .into_iter()
                .filter_map(Result::ok)
                .filter(|e| e.file_type().is_file())
            {
                let Ok(metadata) = file.metadata() else {
                    let feedback = format!(
                        "Failed to read file metadata: {}, container: {}",
                        file.file_name().to_string_lossy(),
                        &local_container_name
                    );

                    eprintln!("{}", feedback);
                    continue;
                };

                if metadata.len() <= SIZE_THRESHOLD {
                    small_files.push(file);
                } else {
                    large_files.push(file);
                }
            }

            let mut small_batch = Vec::<Blob>::new();

            for file in small_files.iter() {
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

                let blob =
                    Blob::new(name, length, None, digest.0, Location::Local);
                small_batch.push(blob);
            }

            sender
                .send(Message::Blobs {
                    container: remote_container_name.clone(),
                    blobs: small_batch,
                    location: Location::Local,
                })
                .expect("Failed to fetch local blobs.");

            ctx.request_repaint();

            for file in large_files.iter() {
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

                let blob = vec![Blob::new(
                    name,
                    length,
                    None,
                    digest.0,
                    Location::Local,
                )];

                sender
                    .send(Message::Blobs {
                        container: remote_container_name.clone(),
                        blobs: blob,
                        location: Location::Local,
                    })
                    .expect("Failed to fetch local blobs.");

                ctx.request_repaint();
            }
        });
    }

    pub fn dispatch_fetch_blob(
        &self,
        ctx: &Context,
        container: &CurrentContainer,
        blob: &Blob,
    ) {
        let sender = self.sender.clone();
        let client = Arc::clone(&self.client);
        let ctx = ctx.clone();

        let blob = blob.clone();
        let remote_container_name = container.name.to_owned();
        let local_container =
            container.local_container(&self.account.local_path);

        shared::println!("%tFetching blob: %n{}/{}", container.name, blob.name);

        self.runtime.spawn(async move {
            shared::println!(
                "%tLocation status: %n{}",
                blob.location
            );

            let bytes: Vec<u8> = 'bytes_block: {

                if blob.location != Location::Remote {

                    if let Some((_, container_path)) = local_container {
                        let file_path = WalkDir::new(&container_path)
                            .into_iter()
                            .filter_map(Result::ok)
                            .filter(|e| e.file_type().is_dir())
                            .find_map(|e| {
                                let path = Path::new(e.path()).join(&blob.name);

                                if path.try_exists().unwrap_or(false) {
                                    Some(path)
                                } else {
                                    None
                                }
                            });

                        if let Some(valid_file_path) = file_path {
                            let bytes = fs::read(valid_file_path)
                                .await
                                .expect("Failed to read local file.");

                            break 'bytes_block bytes;
                        } else {
                            shared::println!(
                                "%tBlob not found locally. Defaulting to remote",
                            );
                        };
                    } else {
                        shared::println!(
                            "%wNo local directory found for container: %n{}%t, fetching from remote instead.",
                            &remote_container_name
                        );
                    };
                };

                let response = client
                    .blob_client(&remote_container_name, &blob.name)
                    .download(None)
                    .await
                    .unwrap();

                response
                    .body
                    .collect()
                    .await
                    .expect("Failed to read blob bytes.")
                    .to_vec()
            };

            let blob = Blob::new(
                blob.name,
                blob.length,
                Some(bytes),
                blob.md5,
                blob.location,
            );

            println!();

            sender
                .send(Message::BlobWithBytes(blob))
                .expect("Failed to fetch blob bytes.");

            ctx.request_repaint();
        });
    }
}
