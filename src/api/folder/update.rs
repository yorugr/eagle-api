use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::{data::FolderId, EagleApi, EagleResponse, Result};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum FolderColor {
    Red,
    Orange,
    Green,
    Yellow,
    Aqua,
    Blue,
    Purple,
    Pink,
}

#[derive(Debug, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct FolderUpdateRequest {
    #[builder(setter(into))]
    folder_id: FolderId,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    new_name: Option<String>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    new_description: Option<String>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    new_color: Option<FolderColor>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct FolderUpdateResponse {
    id: FolderId,
}

impl EagleApi {
    pub async fn folder_update(&self, request: FolderUpdateRequest) -> Result<FolderId> {
        let url = format!("{}/api/folder/update", self.inner.host);

        let resp: EagleResponse<FolderUpdateResponse> = self
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
    async fn test_name() {
        let api = EagleApi::new(&var("EAGLE_API_TEST_HOST").unwrap());
        let folders = api.folder_list().await.unwrap();
        let resp = api
            .folder_update(
                FolderUpdateRequest::builder()
                    .folder_id(folders[0].id.clone())
                    .new_color(FolderColor::Red)
                    .build(),
            )
            .await;
        resp.unwrap();
    }
}
