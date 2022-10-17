use std::net::Ipv4Addr;
use std::error::Error;
use std::env;

use serde::Deserialize;

use reqwest::blocking::Client;

const API_URL: &'static str = "https://api.cloudflare.com/client/v4";
const IP_URL: &'static str = "https://ipv4.icanhazip.com/";

#[derive(Debug, Deserialize, Clone)]
pub struct DnsRecord {
    pub id: String,
    pub content: Ipv4Addr
}

#[derive(Deserialize)]
struct ZoneResponse {
    result: Option<Vec<DnsRecord>>,
    success: bool
}

pub fn get_record(name: &str) -> Result<DnsRecord, Box<dyn Error>> {
    let zone_id = env::var("ZONE_ID")?;
    let api_key = env::var("API_KEY")?;
    let url = format!("{API_URL}/zones/{zone_id}/dns_records?name={name}");

    let client = Client::new();
    let body = client.get(url)
        .header("Authorization", format!("Bearer {api_key}"))
        .send()?
        .text()?;

    let res: ZoneResponse = serde_json::from_str(&body)?;

    if !res.success || res.result.is_none() || res.result.as_ref().unwrap().len() == 0 {
        return Err("Failed to get DNS record".into());
    }

    Ok(res.result.as_ref().unwrap()[0].clone())
}

pub fn set_record(record: DnsRecord) -> Result<(), Box<dyn Error>> {
    let zone_id = env::var("ZONE_ID")?;
    let api_key = env::var("API_KEY")?;
    let url = format!("{API_URL}/zones/{zone_id}/dns_records/{}", record.content);
    
    let client = Client::new();
    client.get(url)
        .header("Authorization", format!("Bearer {api_key}"))
        .send()?;

    Ok(())
}

pub fn get_ip() -> Result<Ipv4Addr, Box<dyn Error>> {
    Ok(reqwest::blocking::get(IP_URL)?.text()?.parse::<Ipv4Addr>()?)
}
