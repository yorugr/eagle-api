use crate::{data::Folder, EagleApi, EagleResponse, Result};

impl EagleApi {
    pub async fn folder_list_recent(&self) -> Result<Vec<Folder>> {
        let url = format!("{}/api/folder/listRecent", self.inner.host);

        let resp: EagleResponse<Vec<Folder>> =
            self.inner.client.get(&url).send().await?.json().await?;

        resp.ok()
    }
}

#[cfg(test)]
mod tests {
    use std::env::var;

    use super::*;

    #[tokio::test]
    async fn test_folder_list_recent() {
        let api = EagleApi::new(&var("EAGLE_API_TEST_HOST").unwrap());
        let resp = api.folder_list_recent().await;
        resp.unwrap();
    }
}
