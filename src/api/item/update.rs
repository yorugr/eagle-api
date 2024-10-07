use serde::Serialize;
use typed_builder::TypedBuilder;

use crate::{
    data::{Item, ItemId},
    EagleApi, EagleResponse, Result,
};

#[derive(Debug, Serialize, TypedBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ItemUpdateRequest {
    #[builder(setter(into))]
    id: ItemId,
    #[builder(default, setter(into))]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tags: Vec<String>,
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    annotation: Option<String>,
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[builder(default, setter(strip_option, into))]
    #[serde(skip_serializing_if = "Option::is_none")]
    star: Option<u8>,
}

impl EagleApi {
    pub async fn item_update(&self, request: ItemUpdateRequest) -> Result<Item> {
        let url = format!("{}/api/item/update", self.inner.host);

        let resp: EagleResponse<Item> = self
            .inner
            .client
            .post(&url)
            .json(&request)
            .send()
            .await?
            .json()
            .await?;

        resp.ok()
    }
}

#[cfg(test)]
mod tests {
    use std::env::var;

    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_item_update() {
        let api = EagleApi::new(&var("EAGLE_API_TEST_HOST").unwrap());
        let list = api.item_list(None).await.unwrap();
        let id = list[0].id.clone();
        let resp = api
            .item_update(
                ItemUpdateRequest::builder()
                    .id(id)
                    .star(5)
                    .annotation("annotation")
                    .build(),
            )
            .await;
        resp.unwrap();
    }
}
