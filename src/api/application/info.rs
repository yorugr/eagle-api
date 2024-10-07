use crate::{data::Application, EagleApi, EagleResponse, Result};

impl EagleApi {
    pub async fn app_info(&self) -> Result<Application> {
        let url = format!("{}/api/application/info", self.inner.host);

        let resp: EagleResponse<Application> =
            self.inner.client.get(&url).send().await?.json().await?;

        resp.ok()
    }
}

#[cfg(test)]
mod tests {
    use std::env::var;

    use super::*;

    #[tokio::test]
    async fn test_app_info() {
        let api = EagleApi::new(&var("EAGLE_API_TEST_HOST").unwrap());
        let resp = api.app_info().await;
        resp.unwrap();
    }
}
