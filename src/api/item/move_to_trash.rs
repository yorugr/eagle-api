use serde::Serialize;

use crate::{data::ItemId, EagleApi, EagleResponse, Result};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ItemMoveToTrashRequest {
    item_ids: Vec<ItemId>,
}

impl EagleApi {
    pub async fn item_move_to_trash(&self, item_ids: impl Into<Vec<ItemId>>) -> Result<()> {
        let url = format!("{}/api/item/moveToTrash", self.inner.host);

        let query = ItemMoveToTrashRequest {
            item_ids: item_ids.into(),
        };

        let resp: EagleResponse<()> = self
            .inner
            .client
            .post(&url)
            .json(&query)
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
    async fn test_item_move_to_trash() {
        let api = EagleApi::new(&var("EAGLE_API_TEST_HOST").unwrap());
        let list = api.item_list(None).await.unwrap();
        let resp = api
            .item_move_to_trash(vec![list[0].id.clone(), list[1].id.clone()])
            .await;
        resp.unwrap();
    }
}
