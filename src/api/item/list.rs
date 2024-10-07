use serde::{Deserialize, Serialize};
use typed_builder::TypedBuilder;

use crate::{data::Item, EagleApi};
use crate::{EagleResponse, Result};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ItemListOrderBy {
    #[serde(rename = "CREATEDATE")]
    CreateDate,
    #[serde(rename = "FILESIZE")]
    FileSize,
    #[serde(rename = "NAME")]
    Name,
    #[serde(rename = "RESOLUTION")]
    Resolution,
    #[serde(rename = "-CREATEDATE")]
    CreateDateDesc,
    #[serde(rename = "-FILESIZE")]
    FileSizeDesc,
    #[serde(rename = "-NAME")]
    NameDesc,
    #[serde(rename = "-RESOLUTION")]
    ResolutionDesc,
}

#[derive(Default, Debug, Clone, Serialize, TypedBuilder)]
pub struct ItemListQuery {
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<usize>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<usize>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    order_by: Option<ItemListOrderBy>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    keyword: Option<String>,
    #[builder(default, setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    ext: Option<String>,
    #[builder(default, setter(into))]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(serialize_with = "crate::utils::comma_separated")]
    tags: Vec<String>,
    #[builder(default, setter(into))]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(serialize_with = "crate::utils::comma_separated")]
    folders: Vec<String>,
}

impl EagleApi {
    pub async fn item_list(&self, query: impl Into<Option<ItemListQuery>>) -> Result<Vec<Item>> {
        let url = format!("{}/api/item/list", self.inner.host);

        let query = query.into().unwrap_or_default();

        let resp: EagleResponse<Vec<Item>> = self
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
    async fn test_item_list() {
        let api = EagleApi::new(&var("EAGLE_API_TEST_HOST").unwrap());
        let resp = api
            .item_list(
                ItemListQuery::builder()
                    .order_by(ItemListOrderBy::CreateDateDesc)
                    .build(),
            )
            .await;
        resp.unwrap();
    }
}
