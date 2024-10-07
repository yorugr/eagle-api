use crate::{EagleApiCode, EagleApiStatus};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error("eagle api status is success but data is missing.")]
    MissingData,
    #[error("eagle api {status:?}. data={data:?}, code={code:?}, message={message:?}")]
    EagleApi {
        status: EagleApiStatus,
        data: Option<String>,
        code: Option<EagleApiCode>,
        message: Option<String>,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
