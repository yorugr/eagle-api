use bytes::Bytes;
use serde::Serialize;

use crate::{EagleApi, Result};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct LibraryIconQuery {
    library_path: String,
}

impl EagleApi {
    pub async fn library_icon(&self, path: impl Into<String>) -> Result<Bytes> {
        let url = format!("{}/api/library/icon", self.inner.host);

        let query = LibraryIconQuery {
            library_path: path.into(),
        };

        let resp = self
            .inner
            .client
            .get(&url)
            .query(&query)
            .send()
            .await?
            .bytes()
            .await?;

        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_library_icon() {
        let api = EagleApi::new(&std::env::var("EAGLE_API_TEST_HOST").unwrap());
        let libs = api.library_history().await.unwrap();
        let icon = api.library_icon(libs[0].clone()).await;
        icon.unwrap();
    }
}
