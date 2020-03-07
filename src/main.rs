extern crate clap;
extern crate hyper;
extern crate hyper_tls;
extern crate serde;
extern crate serde_json;

use std::{thread};
use hyper::Client;
use hyper_tls::HttpsConnector;
use hyper::{Request, Body};
use clap::{Arg, App, SubCommand, AppSettings};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct CloudflareDNSUpdateRequest {
    r#type: String,
    name: String,
    content: String,
    proxied: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let matches = App::new("Cloudflare DNS Updater")
        .version("1.0")
        .author("Eden Reich <eden.reich@gmail.com>")
        .about("Update a DNS records on Cloudflare with dynamic public IP-Address")
        .usage("cloudflare update --token [ACCESS_TOKEN] --zone [ZONE_ID] --dns [DNS_LIST..]")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("update")
            .about("Updates a list of DNS with public ip-address")
            .arg(Arg::with_name("dns")
                .short("d")
                .long("dns")
                .required(true)
                .takes_value(true)
                .multiple(true)
                .value_name("DNS DNS...")
                .help("List of dns to update"))
            .arg(Arg::with_name("token")
                .short("t")
                .long("token")
                .required(true)
                .takes_value(true)
                .value_name("TOKEN")
                .help("API access token"))
            .arg(Arg::with_name("zone")
                .short("z")
                .long("zone")
                .required(true)
                .takes_value(true)
                .value_name("ZONE")
                .help("Zone id")))
        .get_matches();

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let mut ip_address: String = "127.0.0.1".to_string();
    let timeout: u64 = 2;
    let geoip_api_endpoint = "http://ifconfig.me/ip";
    let update_command = matches.subcommand_matches("update").unwrap();
    let cloudflare_zone_id = update_command.value_of("zone").unwrap();
    let cloudflare_api_token = update_command.value_of("token").unwrap();
    let cloudflare_dns_list =  update_command.values_of_lossy("dns").unwrap();
    let cloudflare_api_endpoint = format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records", cloudflare_zone_id);

    loop {

        let ip_address_request = Request::builder()
            .method("GET")
            .uri(geoip_api_endpoint)
            .header("Content-Type", "application/json")
            .body(Body::empty())
            .expect("request builder to return ip address");

        let mut ip_address_raw_response = client.request(ip_address_request).await?;

        if !ip_address_raw_response.status().is_success() {
            panic!("failed to get a successful response of your public ip address!");
        }

        let ip_address_raw_response = hyper::body::to_bytes(ip_address_raw_response).await?;
        
        println!("are still the same ? {:#?} == {:#?}", ip_address_raw_response, ip_address);

        thread::sleep(std::time::Duration::from_secs(timeout));

        if ip_address_raw_response == ip_address {
            continue;
        }
        
        ip_address = std::str::from_utf8(&ip_address_raw_response).unwrap().to_string();

        println!("Your ip address was updated, current ip is: {}", ip_address);
    }

    Ok(())
}
