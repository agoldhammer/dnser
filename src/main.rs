use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::net::IpAddr;
use std::path::PathBuf;
use std::time::Duration;

use hickory_resolver::TokioAsyncResolver;

use tokio::sync::mpsc;
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
        // write!(f, "ip: {}: host: {}", self.ip_addr, self.ptr_records[0])
        write!(f, "ip: {}: ", self.ip_addr).unwrap();
        // for rec in self.ptr_records {
        //     write!(f, "host {}", rec);
        // }
        let results = self
            .ptr_records
            .iter()
            .map(|record| write!(f, "host: {}", record))
            .collect();
        results
    }
}

// Do reverse lookup on ip_str, send result out on channel tx
async fn get_name(ip_str: &String, tx: mpsc::Sender<RevLookupData>) -> () {
    // async fn get_name(ip_str: &String)  . {
    const TIMEOUT_MS: u64 = 2500;
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
    tx.send(rev_lookup_data).await.expect("should just work");
    ()
}

#[tokio::main]
async fn main() {
    const CHAN_BUF_SIZE: usize = 32;
    let (tx, mut rx) = mpsc::channel(CHAN_BUF_SIZE);
    let mut fpath: PathBuf = PathBuf::new();
    fpath.push("unique_ips_54.txt");
    let lines = read_lines(&fpath).unwrap();
    let mut set = JoinSet::new();
    for ip_str in lines {
        let txa = tx.clone();
        set.spawn(async move { get_name(&ip_str.unwrap(), txa).await });
    }
    drop(tx); // have to drop the original channel that has been cloned for each task
    while let Some(rev_lookup_data) = rx.recv().await {
        println!("rcvd: {}", rev_lookup_data);
    }
    while let Some(res) = set.join_next().await {
        res.expect("join error");
    }
}
