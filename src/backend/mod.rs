use crate::app::Message;

use azure_storage_blob::BlobServiceClient;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender, channel};

mod credentials;
mod fetch_containers;
mod fetch_blobs;

pub struct Backend {
    runtime: tokio::runtime::Runtime,
    sender: Sender<Message>,
    client: Arc<BlobServiceClient>,

    pub receiver: Receiver<Message>,
}

impl Backend {
    pub fn new() -> Self {
        let (sender, receiver) = channel();
        let runtime = tokio::runtime::Runtime::new().unwrap();

        let client = credentials::get_account_service_client();

        Self {
            sender,
            receiver,
            runtime,
            client: Arc::new(client),
        }
    }
}

