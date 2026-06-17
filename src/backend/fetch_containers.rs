use super::{Backend, Message};

use futures::TryStreamExt;
use std::sync::Arc;

impl Backend {
    pub fn list_containers(&self) {
        let sender = self.sender.clone();
        let client = Arc::clone(&self.client);

        self.runtime.spawn(async move {
            let mut pager = client.list_containers(None).unwrap().into_pages();

            let mut containers = Vec::<String>::new();

            while let Some(page) = pager.try_next().await.unwrap() {
                let list = page.into_model().unwrap();

                for item in list.container_items {
                    containers.push(item.name.unwrap());
                }
            }

            sender
                .send(Message::Containers(containers))
                .expect("Failed to fetch containers.");
        });
    }
}
