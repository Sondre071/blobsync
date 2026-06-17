use crate::shared;

use azure_core::http::Url;
use azure_storage_blob::*;
use futures::TryStreamExt;

pub async fn list_containers() -> std::result::Result<Vec<String>, Box<dyn std::error::Error>> {
    let account = shared::statics::get_container_account()?;
    let service_url = Url::parse(&format!("{}?{}", account.endpoint, account.sas))?;

    let service_client = BlobServiceClient::new(service_url, None, None)?;
    let mut pager = service_client.list_containers(None)?.into_pages();

    let mut containers = Vec::<String>::new();

    while let Some(page) = pager.try_next().await? {
        let list = page.into_model()?;

        for item in list.container_items {
            containers.push(item.name.unwrap());
        }
    }

    Ok(containers)
}
