use reqwest::Url;
use std::fmt::Formatter;

use crate::{Error, Result};

#[derive(Eq, Hash, PartialEq)]
pub enum Website {
    AbuelasCounterCom,
    ACoupleCooksCom,
    ClaudiaAbrilComBr,
}

impl Website {
    pub fn from(url: &str) -> Result<Self> {
        let url = match Url::parse(&url).map_err(|err| Error::Parse(err.to_string())) {
            Ok(url) => url,
            Err(_) => return Err(Error::UnknownWebsite),
        };

        let domain = match url.domain() {
            None => return Err(Error::UnknownWebsite),
            Some(domain) => domain,
        };

        match domain {
            "abuelascounter.com" => Ok(Self::AbuelasCounterCom),
            "www.acouplecooks.com" => Ok(Self::ACoupleCooksCom),
            "claudia.abril.com.br" => Ok(Self::ClaudiaAbrilComBr),
            _ => Err(Error::UnknownWebsite),
        }
    }
}

impl std::fmt::Display for Website {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Website::AbuelasCounterCom => write!(f, "abuelascounter.com"),
            Website::ACoupleCooksCom => write!(f, "www.acouplecooks.com"),
            Website::ClaudiaAbrilComBr => write!(f, "claudia.abril.com.br"),
        }
    }
}
