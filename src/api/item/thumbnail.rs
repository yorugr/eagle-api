use serde::Serialize;

use crate::{data::ItemId, EagleApi, EagleResponse, Result};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ItemThumbnailQuery {
    id: ItemId,
}

impl EagleApi {
    pub async fn item_thumbnail(&self, id: impl Into<ItemId>) -> Result<String> {
        let url = format!("{}/api/item/thumbnail", self.inner.host);

        let query = ItemThumbnailQuery { id: id.into() };

        let resp: EagleResponse<()> = self
            .inner
            .client
            .get(&url)
            .query(&query)
            .send()
            .await?
            .json()
            .await?;

        resp.ok_str()
    }
}

#[cfg(test)]
mod tests {
    use std::env::var;

    use super::*;

    #[tokio::test]
    async fn test_item_thumbnail() {
        let api = EagleApi::new(&var("EAGLE_API_TEST_HOST").unwrap());
        let list = api.item_list(None).await.unwrap();
        let id = list[0].id.clone();
        let resp = api.item_thumbnail(id).await;
        resp.unwrap();
    }
}
