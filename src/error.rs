use std::{io, result};

use failure::Fail;
use reqwest;
use serde::Deserialize;
use serde_json::error::Error as SerdeError;

pub type ApiResult<T> = result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "An error reported by Yandex-Checkout in the response body.")]
    YandexCheckout(RequestError),

    #[fail(display = "A networking error communicating with the server")]
    Http(reqwest::Error),

    #[fail(display = "An error reading the response body.")]
    Io(io::Error),

    #[fail(display = "An error serializing/deserializing.")]
    Serde(SerdeError),

    #[fail(display = "An operation not supported (yet?) by this library.")]
    Unsupported(&'static str),
}

impl From<reqwest::Error> for Error {
    fn from(other: reqwest::Error) -> Self {
        Error::Http(other)
    }
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other)
    }
}

impl From<RequestError> for Error {
    fn from(other: RequestError) -> Self {
        Error::YandexCheckout(other)
    }
}

impl From<SerdeError> for Error {
    fn from(other: SerdeError) -> Self {
        Error::Serde(other)
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct RequestError {
    /// The HTTP status in the response.
    #[serde(skip_deserializing)]
    pub http_status: u16,

    /// The type of error returned.
    #[serde(rename = "type")]
    pub error_type: String,

    pub id: String,

    pub code: Option<String>,

    pub description: Option<String>,

    pub parameter: Option<String>,
}
