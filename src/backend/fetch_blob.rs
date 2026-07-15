use super::{Backend, Message};
use crate::app::{Blob, Location};
use crate::shared;

use egui::Context;
use std::path::Path;
use std::sync::Arc;
use tokio::fs;

impl Backend {
    pub fn fetch_blob(&self, ctx: &Context, container: &str, blob: &Blob) {
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
