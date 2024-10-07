use serde::Serialize;

use crate::{EagleApi, EagleResponse, Result};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct LibrarySwitchRequest {
    library_path: String,
}

impl EagleApi {
    pub async fn library_switch(&self, path: impl Into<String>) -> Result<()> {
        let url = format!("{}/api/library/switch", self.inner.host);

        let query = LibrarySwitchRequest {
            library_path: path.into(),
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
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_library_switch() {
        let api = EagleApi::new(&std::env::var("EAGLE_API_TEST_HOST").unwrap());
        let libs = api.library_history().await.unwrap();
        let res = api.library_switch(&libs[0]).await;
        res.unwrap();
    }
}
