use std::env;
use dyndns::api;

fn main() {
    let name = env::args().nth(1).expect("Must provide record name");

    let ip = match api::get_ip() {
        Ok(ip) => {
            eprintln!("IP Address is {}", ip);
            ip
        }
        Err(e) => {
            eprintln!("Failed to get IP: {}", e);
            std::process::exit(1);
        }
    };

    let mut rec = match api::get_record(&name) {
        Ok(rec) => {
            eprintln!("DNS Says {}", rec.content);
            rec
        }
        Err(e) => {
            eprintln!("Failed to query DNS: {}", e);
            std::process::exit(1);
        }
    };

    if rec.content != ip {
        eprintln!("Updating DNS Record...");
        rec.content = ip;

        match api::set_record(rec) {
            Ok(_) => eprintln!("Success!"),
            Err(e) => eprintln!("Failed: {e}")
        }
    } else {
        eprintln!("No Change");
    }
}
