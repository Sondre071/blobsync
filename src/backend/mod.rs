use crate::app::types::Message;
use crate::shared::account::Account;

use azure_storage_blob::BlobServiceClient;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender, channel};

mod impls;

pub struct Backend {
    runtime: tokio::runtime::Runtime,
    sender: Sender<Message>,
    client: Arc<BlobServiceClient>,
    account: Account,

    pub receiver: Receiver<Message>,
}

impl Backend {
    pub fn connect(account: &Account) -> Self {
        let (sender, receiver) = channel();
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let client = account.new_blob_client();

        Self {
            sender,
            receiver,
            runtime,
            account: account.clone(),
            client: Arc::new(client),
        }
    }
}
