use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::PathBuf;

use hickory_resolver::TokioAsyncResolver;
use std::net::IpAddr;
use std::time::Duration;
use tokio::task::JoinSet;
use tokio::time::timeout;

fn read_lines(path: &PathBuf) -> Result<io::Lines<BufReader<File>>, Box<dyn Error + 'static>> {
    let file = File::open(path)?;
    return Ok(io::BufReader::new(file).lines());
}

#[derive(Debug)]
struct RevLookupData {
    ip_addr: IpAddr,
    ptr_records: Vec<String>,
}

impl RevLookupData {
    fn new(ip_addr: IpAddr) -> RevLookupData {
        RevLookupData {
            ip_addr: ip_addr,
            ptr_records: Vec::new(),
        }
    }
}

impl fmt::Display for RevLookupData {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ip: {}: host: {}", self.ip_addr, self.ptr_records[0])
    }
}

async fn get_name(ip_str: &String) -> () {
    // async fn get_name(ip_str: &String)  . {
    const TIMEOUT_MS: u64 = 1500;
    let ip_addr: IpAddr = ip_str.parse().unwrap();
    let resolver = TokioAsyncResolver::tokio_from_system_conf().unwrap();

    let reverse_lookup = resolver.reverse_lookup(ip_addr);
    let timeout_duration = Duration::from_millis(TIMEOUT_MS);
    let lookup_result = timeout(timeout_duration, reverse_lookup).await;
    let mut rev_lookup_data = RevLookupData::new(ip_addr);
    match lookup_result {
        Ok(Ok(lookup_result)) => {
            //successful lookup
            // let rev_lookup_data = RevLookupData {
            // ip_addr,
            rev_lookup_data.ptr_records = lookup_result
                .iter()
                .map(|record| format!("{}", record))
                .collect();
            // };
            // println!("{}", rev_lookup_data);
        }
        Ok(Err(_)) => rev_lookup_data.ptr_records.push("unknown".to_string()), //no PTR records found,
        Err(_) => rev_lookup_data.ptr_records.push("timed out".to_string()),   // lookup timed out
    };
    // TODO: pass to channel
    println!("{}", rev_lookup_data);
    ()
}

#[tokio::main]
async fn main() {
    let mut fpath: PathBuf = PathBuf::new();
    fpath.push("unique_ips_54.txt");
    let lines = read_lines(&fpath).unwrap();
    let mut set = JoinSet::new();
    for ip_str in lines {
        set.spawn(async move { get_name(&ip_str.unwrap()).await });
    }
    while let Some(res) = set.join_next().await {
        res.expect("join error");
    }
}
