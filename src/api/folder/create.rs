use serde::{Deserialize, Serialize};

use crate::{data::FolderId, EagleApi, EagleResponse, Result};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct FolderCreateRequest {
    folder_name: String,
    parent: Option<FolderId>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FolderCreateResponse {
    id: FolderId,
}

impl EagleApi {
    pub async fn folder_create(
        &self,
        name: impl Into<String>,
        parent: impl Into<Option<FolderId>>,
    ) -> Result<FolderId> {
        let url = format!("{}/api/folder/create", self.inner.host);

        let request = FolderCreateRequest {
            folder_name: name.into(),
            parent: parent.into(),
        };

        let resp = self.inner.client.post(&url).json(&request).send().await?;
        println!("{}", resp.text().await?);

        let resp: EagleResponse<FolderCreateResponse> = self
            .inner
            .client
            .post(&url)
            .json(&request)
            .send()
            .await?
            .json()
            .await?;

        Ok(resp.ok()?.id)
    }
}

#[cfg(test)]
mod tests {
    use std::env::var;

    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_folder_create() {
        let api = EagleApi::new(&var("EAGLE_API_TEST_HOST").unwrap());
        let folders = api.folder_list().await.unwrap();
        let resp = api
            .folder_create("test".to_string(), folders[0].id.clone())
            .await;
        resp.unwrap();
    }
}
