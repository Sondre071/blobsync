use crate::shared;

use azure_core::http::Url;
use azure_storage_blob::*;
use futures::TryStreamExt;

pub async fn list_containers() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let account = shared::statics::get_container_account()?;
    let service_url = Url::parse(&format!("{}?{}", account.endpoint, account.sas))?;

    let service_client = BlobServiceClient::new(service_url, None, None)?;
    let mut pager = service_client.list_containers(None)?.into_pages();

    while let Some(page) = pager.try_next().await? {
        let list = page.into_model()?;

        for item in list.container_items {
            println!("{}", item.name.unwrap());
        }
    }

    Ok(())
}
