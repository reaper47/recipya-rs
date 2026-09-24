use std::{
    error, io,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    sync::Arc,
};

use derive_more::From;
use itertools::Itertools;
use reqwest::{
    Response,
    dns::{Addrs, Resolve, Resolving},
    redirect::Policy,
};
use tokio::net::lookup_host;
use tracing::error;
use url::{Host, Url};

use crate::impl_display_as_debug;

/// Wrapper around an HTTP client that is safe against Server Side Request Forgery (SSRF) attacks.
//
// AI usage: Most of the solution has been proposed by Claude Sonnet 5.
pub struct SsrfSafeClient(reqwest::Client);

impl SsrfSafeClient {
    /// Creates a new HTTP client that is safe against Server Side Request Forgery (SSRF) attacks.
    pub fn new() -> Result<Self> {
        let client = reqwest::Client::builder()
            .dns_resolver(Arc::new(SSrfSafeResolver))
            .redirect(Policy::custom(|attempt| {
                if attempt.previous().len() > 10 {
                    return attempt.error("stopped after 10 redirects");
                }
                match validate_ip(attempt.url()) {
                    Ok(()) => attempt.follow(),
                    Err(err) => attempt.error(err),
                }
            }))
            .build()
            .inspect_err(|err| error!(?err, "Failed to create SsrSafeClient"))
            .map_err(|err| Error::ReqwestClient(err.to_string()))?;

        Ok(SsrfSafeClient(client))
    }

    /// Convenience method to make a GET request to a URI.
    pub async fn get(&self, url: Url) -> Result<Response> {
        let res = self.0.get(url).send().await?;
        Ok(res)
    }
}

struct SSrfSafeResolver;

impl Resolve for SSrfSafeResolver {
    fn resolve(&self, name: reqwest::dns::Name) -> Resolving {
        Box::pin(async move {
            let addrs = lookup_host((name.as_str(), 0)).await?.collect_vec();

            if addrs.is_empty() || addrs.iter().any(|addr| is_forbidden_ip(addr.ip())) {
                let err = io::Error::new(io::ErrorKind::PermissionDenied, "forbidden address");
                return Err(Box::new(err) as Box<dyn error::Error + Send + Sync>);
            }
            Ok(Box::new(addrs.into_iter()) as Addrs)
        })
    }
}

/// Validates the given URL, ensuring it is not a forbidden IP.
///
// # AI usage
//
// Most of this function was developed by prompting Claude Sonnet 4.6
// to fix a security advisory I received regarding the /fetch endpoint.
//
// I prompted the AI because I am not a security expert and wanted to get
// this security vulnerability fixed.
pub fn validate_ip(url: &Url) -> Result<()> {
    match url.host() {
        Some(Host::Ipv4(ip)) if is_forbidden_v4(ip) => Err(Error::ForbiddenIP),
        Some(Host::Ipv6(ip)) if is_forbidden_v6(ip) => Err(Error::ForbiddenIP),
        Some(_) => Ok(()),
        None => Err(Error::MissingHost),
    }
}

fn is_forbidden_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => is_forbidden_v4(v4),
        IpAddr::V6(v6) => is_forbidden_v6(v6),
    }
}

const fn is_forbidden_v4(ip: Ipv4Addr) -> bool {
    matches!(
        ip.octets(),
        [127 | 10 | 0 | 224..=239 | 240..=255, ..]  // Loopback, private, unspecified, multicast, reserved
        | [172, 16..=31, ..]                        // Private 172.16.0.0/12
        | [192, 168, ..]                            // Private 192.168.0.0/16
        | [169, 254, ..]                            // Link-local + metadata (169.254.169.254)
        | [100, 64..=127, ..]                       // CGN 100.64.0.0/10
        | [198, 18..=19, ..]                        // Benchmarking
        | [192, 0, 2, _]                            // TEST-NET-1
        | [198, 51, 100, _]                         // TEST-NET-2
        | [203, 0, 113, _]                          // TEST-NET-3
    )
}

fn is_forbidden_v6(ip: Ipv6Addr) -> bool {
    if ip == Ipv6Addr::LOCALHOST || ip == Ipv6Addr::UNSPECIFIED {
        return true;
    }

    let segs = ip.segments();
    matches!(segs[0],
        // Link-local fe80::/10
        0xfe80..=0xfebf |
        // Unique-local fc00::/7 (includes fd00::/8)
        0xfc00..=0xfdff |
        // Multicast ff00::/8
        0xff00..=0xffff
    ) ||
    // IPv4-mapped ::ffff:0:0/96
    ip.to_ipv4_mapped()
        .is_some_and(is_forbidden_v4)
}

/// Result type for errors related to the network.
pub type Result<T> = core::result::Result<T, Error>;

/// Enumeration of errors related to the network.
#[derive(Debug, From)]
pub enum Error {
    DNSResolution(Url),
    ForbiddenIP,
    MissingHost,
    ReqwestClient(String),
    SsrfValidation,

    #[from]
    Reqwest(reqwest::Error),
}

impl_display_as_debug!(Error);

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_validate_ip {
        use std::assert_matches;

        use super::*;

        #[test]
        fn test_rejects_ip_loopback_ok() {
            let url = Url::parse("http://127.0.0.1/admin").unwrap();

            let err = validate_ip(&url).unwrap_err();

            assert_matches!(err, Error::ForbiddenIP);
        }

        #[test]
        fn test_rejects_ip_metadata_endpoint_ok() {
            let url = Url::parse("http://169.254.169.254/latest/meta-data/").unwrap();

            let err = validate_ip(&url).unwrap_err();

            assert_matches!(err, Error::ForbiddenIP);
        }

        #[test]
        fn test_rejects_bracketed_ipv6_loopback_ok() {
            let url = Url::parse("http://[::1]/").unwrap();

            let err = validate_ip(&url).unwrap_err();

            assert_matches!(err, Error::ForbiddenIP);
        }

        #[test]
        fn test_errors_on_missing_host_ok() {
            let url = Url::parse("file:///etc/passwd").unwrap();

            let err = validate_ip(&url).unwrap_err();

            assert_matches!(err, Error::MissingHost);
        }

        #[tokio::test]
        async fn test_rejects_hostname_resolving_to_loopback_ok() -> Result<()> {
            let client = SsrfSafeClient::new()?;

            let err = client
                .get(Url::parse("http://localhost/")?)
                .await
                .unwrap_err();

            assert_matches!(err, Error::Reqwest(_));
            Ok(())
        }

        #[tokio::test]
        async fn test_errors_on_unresolvable_hostname_ok() -> Result<()> {
            let client = SsrfSafeClient::new()?;

            let err = client
                .get(Url::parse(
                    "http://this-domain-should-not-exist-9f3a7c.invalid/",
                )?)
                .await
                .unwrap_err();

            assert_matches!(err, Error::Reqwest(_));
            Ok(())
        }

        #[test]
        fn test_allows_public_ip_ok() {
            let url = Url::parse("http://8.8.8.8/").unwrap();

            let got = validate_ip(&url);

            assert!(got.is_ok());
        }
    }
}
