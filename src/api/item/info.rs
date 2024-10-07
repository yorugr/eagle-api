use serde::{Deserialize, Serialize};

use crate::{data::Item, EagleApi, EagleResponse, Result};

#[derive(Debug, Serialize, Deserialize)]
struct ItemInfoQuery {
    id: String,
}

impl EagleApi {
    pub async fn item_info(&self, id: &str) -> Result<Item> {
        let url = format!("{}/api/item/info", self.inner.host);

        let query = ItemInfoQuery { id: id.to_string() };

        let resp: EagleResponse<Item> = self
            .inner
            .client
            .get(&url)
            .query(&query)
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
    async fn test_item_info() {
        let api = EagleApi::new(&var("EAGLE_API_TEST_HOST").unwrap());
        let list = api.item_list(None).await.unwrap();
        let id = list[0].id.as_str();
        let resp = api.item_info(id).await;
        resp.unwrap();
    }
}
