fn main() {
    println!("Hello, world!");
}

use std::time::Duration;

use async_std::task;
use trust_dns_resolver::config::{ResolverConfig,ResolverOpts};
use trust_dns_resolver::lookup::Lookup;
use trust_dns_resolver::system_conf::read_system_conf;

async fn reverse_lookup(ip:&str) -> Result<String, String> {
  let config = ResolverConfig::from_system_conf(read_system_conf().unwrap()).unwrap();

  let timeout = Duration::from_secs(5);

  let lookup = Lookup::reverse(ip);

  let result = task::spawn_blocking(move || {
    let resolver = Resolver::new(config, ResolverOpts::default()).unwrap();
    resolver.lookup(lookup).await
  });

  match task::timeout(timeout, result).await {
    Ok(Ok(records)) => Ok(records[0].to_string()),
    Ok(Err(e)) => Err(format!("Error resolving: {}", e)),
    Err(_) => Err("Timeout".to_string()),
  }
}