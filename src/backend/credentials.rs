use std::env;
use std::fs;

use azure_core::http::Url;
use azure_storage_blob::BlobServiceClient;

pub fn get_storage_accounts() -> Option<Vec<Account>> {
    let file_content = {
        let path = env::home_dir()
            .unwrap()
            .join("AppData/Local/BlobSync/credentials.txt");

        fs::read_to_string(path).expect("Unable to fetch credentials.txt.")
    };

    let mut accounts = Vec::<Account>::new();

    for line in file_content.lines() {
        let mut blob_endpoint: Option<String> = None;
        let mut sas: Option<String> = None;

        for part in line.trim().split(';') {
            let Some((key, value)) = part.split_once('=') else {
                continue;
            };

            match key.trim() {
                "BlobEndpoint" => blob_endpoint = Some(value.trim().to_string()),
                "SharedAccessSignature" => sas = Some(value.trim().to_string()),
                _ => {}
            }
        }

        let (Some(blob_endpoint), Some(sas)) = (blob_endpoint, sas) else {
            panic!("Failed to parse credential.");
        };

        let name = blob_endpoint
            .trim_start_matches("https://")
            .split('.')
            .next()
            .unwrap_or_default()
            .to_string();

        accounts.push(Account {
            name,
            blob_endpoint,
            sas,
        });
    }

    Some(accounts)
}

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
