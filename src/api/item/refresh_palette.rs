use serde::Serialize;

use crate::{data::ItemId, EagleApi, EagleResponse, Result};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ItemRefreshPaletteQuery {
    id: ItemId,
}

impl EagleApi {
    pub async fn item_refresh_palette(&self, id: impl Into<ItemId>) -> Result<()> {
        let url = format!("{}/api/item/refreshPalette", self.inner.host);

        let query = ItemRefreshPaletteQuery { id: id.into() };

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
    async fn test_item_refresh_palette() {
        let api = EagleApi::new(&var("EAGLE_API_TEST_HOST").unwrap());
        let list = api.item_list(None).await.unwrap();
        let id = list[0].id.clone();
        let resp = api.item_refresh_palette(id).await;
        resp.unwrap();
    }
}
