use divan::{Bencher, black_box};

use support::net::validate_ip;
use url::Url;

fn main() {
    divan::main();
}

#[divan::bench(args = [
    "http://127.0.0.1/admin",                   // ipv4 loopback
    "http://169.254.169.254/latest/meta-data/", // metadata endpoint
    "http://[::1]/",                            // ipv6 loopback
    "file:///etc/passwd",                       // missing host
    "http://localhost/",                        // resolves to loopback
    "http://domain-not-exist.invalid/",         // unresolvable hostname
    "http://8.8.8.8/",                          // public ip
    "http://203.0.113.5/",                      // TEST-NET-3
])]
fn validate_ip_bench(bencher: Bencher, url: &str) {
    let url = Url::parse(url).unwrap();
    bencher.bench_local(|| validate_ip(black_box(&url)));
}
