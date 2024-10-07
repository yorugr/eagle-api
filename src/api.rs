use serde::{Deserialize, Serialize};
use serde_json::Value;

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
    Any(Value),
}

impl<T> EagleApiData<T> {
    pub(crate) fn into_message(self) -> Option<String> {
        match self {
            EagleApiData::Message(message) => Some(message),
            EagleApiData::Any(value) => serde_json::to_string(&value).ok(),
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
#[serde(untagged)]
pub enum EagleApiCode {
    String(String),
    Number(i64),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct EagleResponse<T> {
    status: EagleApiStatus,
    data: Option<EagleApiData<T>>,
    code: Option<EagleApiCode>,
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

    pub(crate) fn ok_without_data(self) -> Result<()> {
        match self.status {
            EagleApiStatus::Success => Ok(()),
            _ => Err(Error::EagleApi {
                status: self.status,
                data: self.data.and_then(|data| data.into_message()),
                code: self.code,
                message: self.message,
            }),
        }
    }

    pub(crate) fn ok_str(self) -> Result<String> {
        match self.status {
            EagleApiStatus::Success => match self.data.and_then(|data| data.into_message()) {
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
