# Notes

chat gpt4 suggestion

```rust
use std::net::IpAddr;
use tokio::runtime::Runtime;
use trust_dns_resolver::AsyncResolver;
use trust_dns_resolver::config::\*;

async fn reverse_lookup(ip: IpAddr) {
let resolver = AsyncResolver::tokio_from_system_conf().unwrap();
match resolver.reverse_lookup(ip).await {
Ok(names) => {
for name in names {
println!("{}", name.to_utf8());
}
}
Err(e) => eprintln!("Failed to reverse lookup ip {}: {}", ip, e),
}
}

fn main() {
let rt = Runtime::new().unwrap();
rt.block_on(reverse_lookup("8.8.8.8".parse().unwrap()));
}

with timeout

[dependencies]
tokio = { version = "1", features = ["full"] }
trust-dns-resolver = "0.21.0-alpha.1"

use std::net::IpAddr;
use std::time::Duration;
use tokio::time::timeout;
use trust_dns_resolver::AsyncResolver;
use trust_dns_resolver::config::\*;
use trust_dns_resolver::proto::rr::RData;
use trust_dns_resolver::proto::rr::RecordType;

#[tokio::main]
async fn main() {
let resolver = AsyncResolver::tokio_from_system_conf().unwrap();
let ip_addr: IpAddr = "8.8.8.8".parse().unwrap();

    let reverse_lookup = resolver.reverse_lookup(ip_addr);
    let timeout_duration = Duration::from_secs(5);
    let result = timeout(timeout_duration, reverse_lookup).await;

    match result {
        Ok(Ok(lookup_result)) => {
            for rdata in lookup_result.iter().filter_map(RData::PTR) {
                println!("PTR: {}", rdata.ptrdname());
            }
        }
        Ok(Err(err)) => println!("Failed to find PTR record: {}", err),
        Err(_) => println!("Reverse lookup timed out"),
    }

}

-------------------------

This was the Amazon Q version:
fn main() {
    println!("Hello, world!");
}

use std::net::IpAddr;
use std::time::Duration;

use async_std::{task, future};
// use async_std::io::timeout;
use hickory_resolver::Resolver;
// use hickory_resolver::config::*;
// use hickory_resolver::lookup::Lookup;

// use hickory_resolver::system_conf::read_system_conf;

async fn reverse_lookup(ip:&str) -> Result<String, String> {
//   let config = ResolverConfig::from_system_conf(read_system_conf().unwrap()).unwrap();
  let timeout_ms = Duration::from_millis(50);
  let ip_str = "64.62.197.143";
  let ip: IpAddr = ip_str.parse().unwrap();

  let result = task::spawn_blocking(move || {
      let resolver = Resolver::from_system_conf().unwrap();
      let lookup = resolver.reverse_lookup(ip);
    // let resolver = Resolver::new(config, ResolverOpts::default()).unwrap();
    // resolver.lookup(lookup).await;
  });

  match task::timeout(timeout, result).await {
    Ok(Ok(records)) => Ok(records[0].to_string()),
    Ok(Err(e)) => Err(format!("Error resolving: {}", e)),
    Err(_) => Err("Timeout".to_string()),
  }
}
```
