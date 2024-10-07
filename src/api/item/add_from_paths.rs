use std::collections::HashMap;

use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::{data::FolderId, EagleApi, EagleResponse, Result};

#[derive(Debug, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AddFromPathsItem {
    #[builder(setter(into))]
    path: String,
    #[builder(setter(into))]
    name: String,
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    website: Option<String>,
    #[builder(default, setter(into))]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tags: Vec<String>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    star: Option<u8>,
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    annotation: Option<String>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    modification_time: Option<i64>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    headers: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AddFromPathsRequest {
    #[builder(setter(into))]
    items: Vec<AddFromPathsItem>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    folder_id: Option<FolderId>,
}

impl EagleApi {
    pub async fn item_add_from_paths(&self, request: AddFromPathsRequest) -> Result<()> {
        let url = format!("{}/api/item/addFromPaths", self.inner.host);

        let resp: EagleResponse<()> = self
            .inner
            .client
            .post(&url)
            .json(&request)
            .send()
            .await?
            .json()
            .await?;

        resp.ok_without_data()
    }
}

#[cfg(test)]
mod tests {
    use std::env::var;

    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_item_add_from_paths() {
        let api = EagleApi::new(&var("EAGLE_API_TEST_HOST").unwrap());
        let resp = api
            .item_add_from_paths(
                AddFromPathsRequest::builder()
                    .items([
                        AddFromPathsItem::builder()
                            .path("/a.png")
                            .name("a")
                            .tags([])
                            .star(1)
                            .build(),
                        AddFromPathsItem::builder()
                            .path("/b.png")
                            .name("b")
                            .tags([])
                            .build(),
                    ])
                    .build(),
            )
            .await;
        resp.unwrap();
    }
}
