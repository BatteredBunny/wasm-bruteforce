use std::{
    error::Error,
    fmt::{Display, Formatter},
    str::FromStr,
};

use gloo::{
    file::{Blob, ObjectUrl},
    net::http::Request,
};
use web_sys::RequestRedirect;

use crate::{
    code_generation::CodeGenerator,
    site::{FetchError, FetchResult},
};

const IMGUR_DEFAULT_LINK_LENGTH: f64 = 5.0;
const LIGHTSHOT_DEFAULT_LINK_LENGTH: f64 = 6.0;

#[derive(Debug, Clone, PartialEq)]
pub enum Sites {
    Imgur,
    Lightshot,
}

impl Sites {
    pub fn default_length(&self) -> f64 {
        match self {
            Sites::Imgur => IMGUR_DEFAULT_LINK_LENGTH,
            Sites::Lightshot => LIGHTSHOT_DEFAULT_LINK_LENGTH,
        }
    }

    pub async fn fetch(
        &self,
        code_generator: &mut CodeGenerator,
    ) -> Result<FetchResult, FetchError> {
        match self {
            Sites::Imgur => {
                let request =
                    Request::new(code_generator.generate()).redirect(RequestRedirect::Manual);
                let response = request.send().await?;
                match response.status() {
                    429 => Err(FetchError::RateLimited),
                    302 | 0 => Err(FetchError::InvalidImage),
                    200 => Ok(FetchResult {
                        blob: ObjectUrl::from(Blob::new(&*response.binary().await?)),
                        origin_url: response.url(),
                    }),
                    _ => Err(FetchError::Unexpected),
                }
            }
            Sites::Lightshot => { // TODO: implement
                gloo::dialogs::alert("Unfinished xp");
                Err(FetchError::RateLimited)
            },
        }
    }
}
#[derive(Debug, Clone)]
pub enum SitesError {
    InvalidSite,
}

impl Error for SitesError {}

impl Display for SitesError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            SitesError::InvalidSite => writeln!(f, "Invalid site"),
        }
    }
}

impl FromStr for Sites {
    type Err = SitesError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "imgur" => Ok(Sites::Imgur),
            "lightshot" => Ok(Sites::Lightshot),
            _ => Err(SitesError::InvalidSite),
        }
    }
}
