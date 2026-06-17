use super::{Backend, Message};

use futures::TryStreamExt;
use std::sync::Arc;
use egui::Ui;

impl Backend {
    pub fn list_blobs(&self, ui: &Ui, container: &str) {
        let sender = self.sender.clone();
        let client = Arc::clone(&self.client);
        let container = container.to_string();
        let ctx = ui.ctx().clone();

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
            
            ctx.request_repaint();
        });
    }
}

impl Backend {
    pub fn fetch_blob(&self, ui: &Ui, container: &str, name: &str) {
        let sender = self.sender.clone();
        let client = Arc::clone(&self.client);
        let container = container.to_string();
        let name = name.to_string();
        let ctx = ui.ctx().clone();
        
        println!("Fetching blob: {}, container: {}.", name, container);

        self.runtime.spawn(async move {
            let response = client
                .blob_client(&container, &name)
                .download(None)
                .await
                .unwrap();

            let bytes: Vec<u8> = response
                .body
                .collect()
                .await
                .expect("Failed to parse blob bytes.")
                .to_vec();
            
            println!("Parsed bytes.");

            sender
                .send(Message::Blob {
                    name,
                    container,
                    bytes,
                })
                .expect("Failed to download blob.");
            
            ctx.request_repaint();
        });
    }
}
