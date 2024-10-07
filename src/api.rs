use serde::{Deserialize, Serialize};

use crate::{Error, Result};

pub mod application;
pub mod folder;
pub mod item;
pub mod library;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum EagleApiStatus {
    Success,
    Error,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum EagleApiData<T> {
    Message(String),
    Data(T),
}

impl<T> EagleApiData<T> {
    pub(crate) fn into_message(self) -> Option<String> {
        match self {
            EagleApiData::Message(message) => Some(message),
            _ => None,
        }
    }

    pub(crate) fn into_data(self) -> Option<T> {
        match self {
            EagleApiData::Data(data) => Some(data),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct EagleResponse<T> {
    status: EagleApiStatus,
    data: Option<EagleApiData<T>>,
    code: Option<i64>,
    message: Option<String>,
}

impl<T> EagleResponse<T> {
    pub(crate) fn ok(self) -> Result<T> {
        match self.status {
            EagleApiStatus::Success => match self.data.and_then(|data| data.into_data()) {
                Some(data) => Ok(data),
                None => Err(Error::MissingData),
            },
            _ => Err(Error::EagleApi {
                status: self.status,
                data: self.data.and_then(|data| data.into_message()),
                code: self.code,
                message: self.message,
            }),
        }
    }
}
