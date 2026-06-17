use core::panic;
use std::env;
use std::fs;

use azure_core::http::Url;
use azure_storage_blob::BlobServiceClient;

pub fn get_account_service_client() -> BlobServiceClient {
    let file_content = {
        let path = env::home_dir()
            .unwrap()
            .join("AppData/Local/BlobSync/connectionstring.txt");

        fs::read_to_string(path).unwrap()
    };

    let service_url: Url = {
        let mut blob_endpoint: Option<String> = None;
        let mut sas: Option<String> = None;

        for part in file_content.trim().split(';') {
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
            panic!("Unable to fetch storage account credentials")
        };

        Url::parse(&format!("{}?{}", blob_endpoint, sas))
    }
    .expect("Unable to parse URL.");

    BlobServiceClient::new(service_url, None, None).expect("Unable to create blob service client.")
}
