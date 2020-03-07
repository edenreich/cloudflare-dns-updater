extern crate clap;
extern crate hyper;
extern crate hyper_tls;

use std::{thread};
use hyper::Client;
use hyper_tls::HttpsConnector;
use hyper::{Request, Body};
use clap::{Arg, App, SubCommand, AppSettings};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct CloudflareDNSUpdateRequest {
    r#type: String,
    name: String,
    content: String,
    proxied: bool,
}

#[derive(Serialize, Deserialize)]
struct GeoIpAddressResponse {
    status: String,
    query: String,
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
    let mut ip_address = "127.0.0.1";
    let geoip_api_endpoint = "http://ip-api.com/json";
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

        let ip_address_response = client.request(ip_address_request).await?;

        if !ip_address_response.status().is_success() {
            panic!("failed to get a successful response of your public ip address!");
        }

        let body = ip_address_response.into_body();
        // let ip_address = serde_json::from_str(&body);
        // let val: Value = serde_json::from_slice(&body);

        println!("{:#?}", body);

        thread::sleep(std::time::Duration::from_secs(2));
    }

    // let response = client.request(req1).await?;

    // let buf = hyper::body::to_bytes(response).await?;

    // println!("response {:?}", buf);

    Ok(())
}
