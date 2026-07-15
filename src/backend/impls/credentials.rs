/*
 * For later
 */
#[allow(dead_code)]
fn parse_connection_string(connection_string: String) -> Account {
    let mut blob_endpoint: Option<String> = None;
    let mut sas: Option<String> = None;

    for part in connection_string.trim().split(';') {
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

    Account {
        name,
        blob_endpoint,
        sas,
    }
}
