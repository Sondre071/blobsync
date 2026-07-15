use super::{Backend, Message};
use crate::app::{Blob, Location};

use egui::Context;
use futures::TryStreamExt;
use std::sync::Arc;

impl Backend {
    pub(super) fn fetch_remote_container(&self, ctx: &Context, container: &str) {
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
}
