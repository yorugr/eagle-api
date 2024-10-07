use serde::{Deserialize, Serialize};

use crate::{data::FolderId, EagleApi, EagleResponse, Result};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct FolderRenameRequest {
    folder_id: FolderId,
    new_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FolderRenameResponse {
    id: FolderId,
}

impl EagleApi {
    pub async fn folder_rename(
        &self,
        folder_id: impl Into<FolderId>,
        new_name: impl Into<String>,
    ) -> Result<FolderId> {
        let url = format!("{}/api/folder/rename", self.inner.host);

        let request = FolderRenameRequest {
            folder_id: folder_id.into(),
            new_name: new_name.into(),
        };

        let resp: EagleResponse<FolderRenameResponse> = self
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
    async fn test_folder_rename() {
        let api = EagleApi::new(&var("EAGLE_API_TEST_HOST").unwrap());
        let folders = api.folder_list().await.unwrap();
        let resp = api
            .folder_rename(folders[0].id.clone(), "new name".to_string())
            .await;
        resp.unwrap();
    }
}
