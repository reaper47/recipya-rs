use std::fmt::Formatter;
use reqwest::Url;

use crate::{Error, Result};

#[derive(Eq, Hash, PartialEq)]
pub enum Website {
    AbuelasCounterCom,
    ClaudiaAbrilComBr,
    Unknown,
}

impl Website {
    pub fn from(url: &str) -> Result<Self> {
        let url = match Url::parse(&url).map_err(|err| Error::Parse(err.to_string())) {
            Ok(url) => url,
            Err(_) => return Err(Error::UnknownWebsite),
        };

        let host = match url.host_str() {
            None => return Err(Error::UnknownWebsite),
            Some(host) => host,
        };

        match host {
            "abuelascounter.com" => Ok(Self::AbuelasCounterCom),
            "claudia.abril.com.br" => Ok(Self::ClaudiaAbrilComBr),
            _ => Err(Error::UnknownWebsite),
        }
    }
}

impl std::fmt::Display for Website {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Website::AbuelasCounterCom => write!(f, "abuelascounter.com"),
            Website::ClaudiaAbrilComBr => write!(f, "claudia.abril.com.br"),
            Website::Unknown => write!(f, "unknown"),
        }
    }
}
