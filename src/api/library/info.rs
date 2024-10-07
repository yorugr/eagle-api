use crate::{data::Library, EagleApi, EagleResponse, Result};

impl EagleApi {
    pub async fn library_info(&self) -> Result<Library> {
        let url = format!("{}/api/library/info", self.inner.host);

        let resp: EagleResponse<Library> = self.inner.client.get(&url).send().await?.json().await?;

        resp.ok()
    }
}

#[cfg(test)]
mod tests {
    use std::env::var;

    use super::*;

    #[tokio::test]
    async fn test_library_info() {
        let api = EagleApi::new(&var("EAGLE_API_TEST_HOST").unwrap());
        let resp = api.library_info().await;
        resp.unwrap();
    }
}
