use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use tokio::net::lookup_host;
use url::{Host, Url};

use crate::impl_display_as_debug;

/// Resolves and validates the given URL, ensuring it is not a forbidden IP.
///
/// # AI usage
///
/// Most of this function was developed by prompting Claude Sonnet 4.6
/// to fix a security advisory I received regarding the /fetch endpoint.
///
/// I prompted the AI because I am not a security expert and wanted to get
/// this security vulnerability fixed.
pub async fn resolve_and_validate(url: Url) -> Result<Url> {
    match url.host() {
        Some(Host::Ipv4(ip)) => {
            if is_forbidden_v4(ip) {
                return Err(Error::ForbiddenIP(url));
            }
            Ok(url)
        }
        Some(Host::Ipv6(ip)) => {
            if is_forbidden_v6(ip) {
                return Err(Error::ForbiddenIP(url));
            }
            Ok(url)
        }
        Some(Host::Domain(domain)) => {
            let port = url.port_or_known_default().unwrap_or(80);
            let addrs: Vec<IpAddr> = lookup_host((domain, port))
                .await
                .map_err(|_| Error::DNSResolution(url.clone()))?
                .map(|s| s.ip())
                .collect();

            if addrs.is_empty() {
                return Err(Error::DNSResolution(url));
            }

            if addrs.iter().any(|ip| is_forbidden_ip(*ip)) {
                return Err(Error::ForbiddenIP(url));
            }

            Ok(url)
        }
        None => Err(Error::MissingHost(url)),
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
#[derive(Debug)]
pub enum Error {
    DNSResolution(Url),
    ForbiddenIP(Url),
    MissingHost(Url),
}

impl_display_as_debug!(Error);

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    mod tests_resolve_and_validate {
        use std::assert_matches;

        use super::*;

        #[tokio::test]
        async fn test_rejects_ip_loopback_ok() -> Result<()> {
            let url = Url::parse("http://127.0.0.1/admin")?;

            let err = resolve_and_validate(url).await.unwrap_err();

            assert_matches!(err, Error::ForbiddenIP(_));
            Ok(())
        }

        #[tokio::test]
        async fn test_rejects_ip_metadata_endpoint_ok() -> Result<()> {
            let url = Url::parse("http://169.254.169.254/latest/meta-data/")?;

            let err = resolve_and_validate(url).await.unwrap_err();

            assert_matches!(err, Error::ForbiddenIP(_));
            Ok(())
        }

        #[tokio::test]
        async fn test_rejects_bracketed_ipv6_loopback_ok() -> Result<()> {
            let url = Url::parse("http://[::1]/")?;

            let err = resolve_and_validate(url).await.unwrap_err();

            assert_matches!(err, Error::ForbiddenIP(_));
            Ok(())
        }

        #[tokio::test]
        async fn test_errors_on_missing_host_ok() -> Result<()> {
            let url = Url::parse("file:///etc/passwd")?;

            let err = resolve_and_validate(url).await.unwrap_err();

            assert_matches!(err, Error::MissingHost(_));
            Ok(())
        }

        #[tokio::test]
        async fn test_rejects_hostname_resolving_to_loopback_ok() -> Result<()> {
            let url = Url::parse("http://localhost/")?;

            let err = resolve_and_validate(url).await.unwrap_err();

            assert_matches!(err, Error::ForbiddenIP(_));
            Ok(())
        }

        #[tokio::test]
        async fn test_errors_on_unresolvable_hostname_ok() -> Result<()> {
            let url = Url::parse("http://this-domain-should-not-exist-9f3a7c.invalid/")?;

            let err = resolve_and_validate(url).await.unwrap_err();

            assert_matches!(err, Error::DNSResolution(_));
            Ok(())
        }

        #[tokio::test]
        async fn test_allows_public_ip_ok() -> Result<()> {
            let url = Url::parse("http://8.8.8.8/")?;

            let got = resolve_and_validate(url).await;

            assert!(got.is_ok());
            Ok(())
        }
    }
}
