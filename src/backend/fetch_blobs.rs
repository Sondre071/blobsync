use super::{Backend, Message};

use futures::TryStreamExt;
use std::sync::Arc;

impl Backend {
    pub fn list_blobs(&self, container: &str) {
        let sender = self.sender.clone();
        let client = Arc::clone(&self.client);
        let container = container.to_string();

        self.runtime.spawn(async move {
            let mut pager = client
                .blob_container_client(&container)
                .list_blobs(None)
                .unwrap()
                .into_pages();

            let mut blobs = Vec::<String>::new();

            while let Some(page) = pager.try_next().await.unwrap() {
                let list = page.into_model().unwrap();

                for item in list.blob_items {
                    blobs.push(item.name.unwrap());
                }
            }

            sender
                .send(Message::Blobs { container, blobs })
                .expect("Failed to fetch blobs.");
        });
    }
}
