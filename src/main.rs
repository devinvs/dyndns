use std::env;
use std::time::Duration;
use dyndns::api;

fn main() {
    dotenv::dotenv().unwrap();
    let name = env::args().nth(1).expect("Must provide record name");

    loop {
        let mut failed = true;

        if let Ok(ip) = api::get_ip() {
            if let Ok(rec) = api::get_record(&name) {
                if rec.content != ip {
                    
                } else {
                    failed = false;
                }
            }
        }

        let sleep_dur = if failed {
            // 5 seconds
            Duration::from_secs(5)
        } else {
            // 5 minutes
            Duration::from_secs(5 * 60)
        };

        std::thread::sleep(sleep_dur);
    }
}
