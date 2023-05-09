use std::net::Ipv4Addr;
use std::error::Error;
use std::collections::HashMap;

use serde::Deserialize;

use reqwest::blocking::Client;

const API_URL: &'static str = "https://api.cloudflare.com/client/v4";
const IP_URL: &'static str = "https://ipv4.icanhazip.com/";

const ZONE_ID: &'static str = "2aa8b7223fd8f52c88e141d13dd340e6";
const API_KEY: &'static str = "KyHejnhOanguIdG7rJo9883VWmEmq1tc5r6Svv1l";

#[derive(Debug, Deserialize, Clone)]
pub struct DnsRecord {
    pub id: String,
    pub content: Ipv4Addr
}

#[derive(Debug, Deserialize)]
struct ZoneResponse {
    result: Option<Vec<DnsRecord>>,
    success: bool
}

#[derive(Debug, Deserialize)]
struct PatchResponse {
    result: Option<DnsRecord>,
    success: bool
}

pub fn get_record(name: &str) -> Result<DnsRecord, Box<dyn Error>> {
    let url = format!("{API_URL}/zones/{ZONE_ID}/dns_records?name={name}");

    let client = Client::new();
    let body = client.get(url)
        .header("Authorization", format!("Bearer {API_KEY}"))
        .send()?
        .text()?;

    let res: ZoneResponse = serde_json::from_str(&body)?;

    if !res.success || res.result.is_none() || res.result.as_ref().unwrap().len() == 0 {
        return Err("Failed to get DNS record".into());
    }

    Ok(res.result.as_ref().unwrap()[0].clone())
}

pub fn set_record(record: DnsRecord) -> Result<(), Box<dyn Error>> {
    let url = format!("{API_URL}/zones/{ZONE_ID}/dns_records/{}", record.id);
    let mut data = HashMap::new();
    data.insert("content", record.content.to_string());

    let client = Client::new();
    let body = client.patch(url)
        .header("Authorization", format!("Bearer {API_KEY}"))
        .json(&data)
        .send()?
        .text()?;

    let res: PatchResponse = serde_json::from_str(&body)?;
    if !res.success || res.result.is_none() {
        return Err("Failed to update DNS record".into());
    }

    Ok(())
}

pub fn get_ip() -> Result<Ipv4Addr, Box<dyn Error>> {
    let s = reqwest::blocking::get(IP_URL)?.text()?;
    Ok(s.trim_end().parse::<Ipv4Addr>()?)
}
