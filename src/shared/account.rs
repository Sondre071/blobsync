use super::Shared;
use crate::shared;

use std::{env, fs};

use azure_core::http::Url;
use azure_storage_blob::BlobServiceClient;
use serde::Deserialize;

impl Shared {
    pub fn get_storage_accounts() -> Vec<Account> {
        let path = env::home_dir()
            .unwrap()
            .join("AppData/Local/BlobSync/accounts.json");

        let file = fs::File::open(path).unwrap();
        let reader = std::io::BufReader::new(file);

        let accounts_file: AccountsFile = serde_json::from_reader(reader).unwrap();

        accounts_file.accounts
    }
}

#[derive(Deserialize)]
struct AccountsFile {
    accounts: Vec<Account>,
}

#[derive(Deserialize, Clone)]
pub struct Account {
    pub name: String,
    pub local_path: String,
    blob_endpoint: String,
    sas: String,
}

impl Account {
    pub fn new_blob_client(&self) -> BlobServiceClient {
        let url = Url::parse(&format!("{}?{}", self.blob_endpoint, self.sas))
            .expect("Unable to parse URL.");

        shared::println!("%tCreating new client for storage account: %n{}\n", self.name);

        BlobServiceClient::new(url, None, None).expect("Unable to create blob service client.")
    }
}
