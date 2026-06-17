use std::env;
use std::fs;

pub fn get_container_account() -> Result<StorageAccount, ()> {
    let path = env::home_dir()
        .unwrap()
        .join("AppData/Local/BlobSync/connectionstring.txt");

    let content = fs::read_to_string(path).unwrap();
    let storage_account = parse_connection_string(content);

    Ok(storage_account)
}

fn parse_connection_string(text: String) -> StorageAccount {
    let mut blob_endpoint: Option<String> = None;
    let mut sas: Option<String> = None;

    for part in text.trim().split(';') {
        let Some((key, value)) = part.split_once('=') else {
            continue;
        };

        match key.trim() {
            "BlobEndpoint" => blob_endpoint = Some(value.trim().to_string()),
            "SharedAccessSignature" => sas = Some(value.trim().to_string()),
            _ => {}
        }
    }

    StorageAccount {
        endpoint: blob_endpoint.expect("Failed to parse SAS."),
        sas: sas.expect("Failed to parse blob endpoint."),
    }
}

pub struct StorageAccount {
    pub endpoint: String,
    pub sas: String,
}
