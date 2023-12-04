use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;
use std::error::Error;

use std::net::IpAddr;
use std::time::Duration;
use tokio::time::timeout;
use tokio::task::JoinSet;
use hickory_resolver::TokioAsyncResolver;
// use hickory_resolver::error::ResolveError;
// use tokio::time::error::Elapsed;
// use hickory_resolver::lookup::ReverseLookup;
// use hickory_resolver::config::\*;
// use hickory_resolver::proto::rr::RData;
// use hickory_resolver::proto::rr::RecordType;

fn read_lines(path: &PathBuf) -> Result<io::Lines<BufReader<File>>, Box<dyn Error + 'static> >{
    let file = File::open(path)?;
    return Ok(io::BufReader::new(file).lines());
}

async fn get_name(ip_str: &String)  -> () {
// async fn get_name(ip_str: &String)  . {
    const TIMEOUT_MS: u64 = 1500;
    let ip_addr: IpAddr = ip_str.parse().unwrap();
    let resolver = TokioAsyncResolver::tokio_from_system_conf().unwrap();

    let reverse_lookup = resolver.reverse_lookup(ip_addr);
    let timeout_duration = Duration::from_millis(TIMEOUT_MS);
    let lookup_result = timeout(timeout_duration, reverse_lookup).await;
    match lookup_result {
                Ok(Ok(lookup_result)) => {
                    for record in lookup_result.iter() {
                        println!("ip: {}: host: {}", ip_str, record);
                    }
                }
                Ok(Err(_)) => println!("ip: {}: host: unknown", ip_str),
                Err(_) => println!("ip: {}: timed out", ip_str),
        };
        ()
}

#[tokio::main]
async fn main() {
    let mut fpath: PathBuf = PathBuf::new();
    fpath.push("unique_ips_54.txt");
    let lines = read_lines(&fpath).unwrap();
    let mut set = JoinSet::new();
    for ip_str in lines {
        set.spawn(async move {get_name(&ip_str.unwrap()).await});
    }
    while let Some(res) = set.join_next().await {
        res.expect("join error");
    };
}