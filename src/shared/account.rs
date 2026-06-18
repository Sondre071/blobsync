use super::Shared;

use std::env;
use std::fs;

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

        let account_file: AccountFile = serde_json::from_reader(reader).unwrap();

        account_file.accounts
    }
}

#[derive(Deserialize)]
struct AccountFile {
    accounts: Vec<Account>,
}

#[derive(Deserialize)]
pub struct Account {
    pub name: String,
    blob_endpoint: String,
    sas: String,
}

impl Account {
    pub fn new_blob_client(&self) -> BlobServiceClient {
        let url = Url::parse(&format!("{}?{}", self.blob_endpoint, self.sas))
            .expect("Unable to parse URL.");

        println!("Creating new client for storage account: {}", self.name);

        BlobServiceClient::new(url, None, None).expect("Unable to create blob service client.")
    }
}
