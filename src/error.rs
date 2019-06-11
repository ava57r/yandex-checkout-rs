use std::{io, result};

use failure::Fail;

pub type ApiResult<T> = result::Result<T, Error>;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "A networking error communicating with the server")]
    Http(reqwest::Error),

    #[fail(display = "An error reading the response body.")]
    Io(std::io::Error),

    #[fail(display = "An error serializing a request before it is sent to server.")]
    Serialize,

    #[fail(display = "An error deserializing a response received from server.")]
    Deserialize,

    #[fail(display = "An operation not supported (yet?) by this library.")]
    Unsupported(&'static str),
}
