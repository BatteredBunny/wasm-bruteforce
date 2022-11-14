use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use gloo::file::ObjectUrl;

use crate::{code_generation::CodeGenerator, sites::Sites};

#[derive(Debug)]
pub enum FetchError {
    Request(gloo::net::Error),
    InvalidImage,
    RateLimited,
    Unexpected,
}
impl Error for FetchError {}

impl From<gloo::net::Error> for FetchError {
    fn from(e: gloo::net::Error) -> Self {
        Self::Request(e)
    }
}

impl Display for FetchError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            FetchError::Request(e) => writeln!(f, "Error in the request: {e}"),
            FetchError::InvalidImage => writeln!(f, "invalid image"),
            FetchError::RateLimited => writeln!(f, "Ratelimited"),
            FetchError::Unexpected => writeln!(f, "Unexpected error"),
        }
    }
}

pub struct FetchResult {
    pub blob: ObjectUrl,
    pub origin_url: String,
}

pub struct Site {
    pub site: Sites,
    pub code_length: f64,
    pub code_generator: CodeGenerator,
}

impl Site {
    pub fn new(site: Sites, code_length: f64) -> Self {
        Site {
            code_generator: CodeGenerator::new(&site, code_length as usize),
            site,
            code_length,
        }
    }

    pub async fn fetch(&mut self) -> Result<FetchResult, FetchError> {
        self.site.fetch(&mut self.code_generator).await
    }

    pub fn set_code_length(&mut self, code_length: f64) {
        self.code_length = code_length;
        self.code_generator = CodeGenerator::new(&self.site, code_length as usize);
    }

    pub fn set_site(&mut self, site: Sites) {
        self.site = site;

        self.code_length = self.site.default_length();
        self.code_generator = CodeGenerator::new(&self.site, self.code_length as usize);
    }
}
