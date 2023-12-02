// use std::fs::File;
// use std::io::{self, BufRead};
// use std::path::Path;
use std::net::IpAddr;
use std::time::Duration;
use tokio::time::timeout;
use hickory_resolver::AsyncResolver;
// use hickory_resolver::config::\*;
// use hickory_resolver::proto::rr::RData;
// use hickory_resolver::proto::rr::RecordType;

#[tokio::main]
  async fn main() {
    

    let resolver = AsyncResolver::tokio_from_system_conf().unwrap();
    let ip_addr: IpAddr = "8.8.8.8".parse().unwrap();

    let reverse_lookup = resolver.reverse_lookup(ip_addr);
    let timeout_duration = Duration::from_millis(150);
    let result = timeout(timeout_duration, reverse_lookup).await;

    match result {
        Ok(Ok(lookup_result)) => {
            
            for z in lookup_result.iter() {
                println!("host: {}", z);
            }
            
            // for rdata in lookup_result.iter().filter_map(RData::PTR) {
            //     println!("PTR: {}", rdata.ptrdname());
            // }
            // dbg!(y);
            // dbg!(lookup_result);
        }
        Ok(Err(err)) => println!("Failed to find PTR record: {}", err),
        Err(_) => println!("Reverse lookup timed out"),
    }

}