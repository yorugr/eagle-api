use crate::{EagleApi, EagleResponse, Result};

impl EagleApi {
    pub async fn library_history(&self) -> Result<Vec<String>> {
        let url = format!("{}/api/library/history", self.inner.host);

        let resp: EagleResponse<Vec<String>> =
            self.inner.client.get(&url).send().await?.json().await?;

        resp.ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_library_history() {
        let api = EagleApi::new(&std::env::var("EAGLE_API_TEST_HOST").unwrap());
        let resp = api.library_history().await;
        resp.unwrap();
    }
}
