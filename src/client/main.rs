extern crate clap;

use std::thread;
use clap::{App, AppSettings, Arg, SubCommand};
use hyper::{Body, Client, Request};
use hyper_tls::HttpsConnector;

use cloudflare::{
    typings::{
        DNSListResponse,
        DNSResponse,
        DNSRecordSpec,
    },
    utils::{
        get_ip_address,
    }
};

fn cli() -> App<'static, 'static> {
    App::new("Cloudflare DNS Updater")
        .version("1.0")
        .author("Eden Reich <eden.reich@gmail.com>")
        .about("Update a DNS records on Cloudflare with dynamic public IP-Address")
        .usage("cloudflare update --token [ACCESS_TOKEN] --zone [ZONE_ID] --dns [DNS_LIST..]")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("update")
                .about("Updates a list of DNS with public IP-Address")
                .arg(
                    Arg::with_name("dns")
                        .short("d")
                        .long("dns")
                        .required(true)
                        .takes_value(true)
                        .multiple(true)
                        .value_name("DNS DNS...")
                        .help("List of dns to update"),
                )
                .arg(
                    Arg::with_name("token")
                        .short("t")
                        .long("token")
                        .required(true)
                        .takes_value(true)
                        .env("CLOUDFLARE_ACCESS_TOKEN")
                        .value_name("TOKEN")
                        .help("API access token"),
                )
                .arg(
                    Arg::with_name("zone")
                        .short("z")
                        .long("zone")
                        .required(true)
                        .takes_value(true)
                        .env("CLOUDFLARE_ZONE_ID")
                        .value_name("ZONE")
                        .help("Zone id"),
                )
                .arg(
                    Arg::with_name("intervals")
                        .short("i")
                        .long("intervals")
                        .takes_value(true)
                        .default_value("2")
                        .value_name("INTERVALS")
                        .help("How often to check in seconds"),
                ),
        )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = cli().get_matches();

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let mut current_ip_address: String = "127.0.0.1".to_owned();

    let update_command = matches.subcommand_matches("update").unwrap();

    let intervals: u64 = update_command.value_of("intervals").unwrap().parse::<u64>()?;
    let input_cloudflare_zone_id: String = update_command.value_of("zone").unwrap().to_owned();
    let input_cloudflare_api_token: String = update_command.value_of("token").unwrap().to_owned();
    let input_cloudflare_dns_list: Vec<String> = update_command.values_of_lossy("dns").unwrap();
    let cloudflare_api_dns_endpoint: String = format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records", input_cloudflare_zone_id);

    loop {
        let ip_address = get_ip_address().await.unwrap();
        if current_ip_address == ip_address {
            println!("IP address is the same, skipping update");
            thread::sleep(std::time::Duration::from_secs(intervals));
            continue;
        }
        current_ip_address = ip_address;

        let dns_list_request = Request::builder()
            .method("GET")
            .uri(&cloudflare_api_dns_endpoint)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", input_cloudflare_api_token))
            .body(Body::empty())
            .expect("request builder to return a dns list");

        let dns_raw_response = client.request(dns_list_request).await?;

        if !dns_raw_response.status().is_success() {
            panic!("failed to get a list of dns from cloudflare!");
        }

        let dns_response_content_bytes: &hyper::body::Bytes = &hyper::body::to_bytes(dns_raw_response).await?;
        let dns_response_content = std::str::from_utf8(&dns_response_content_bytes).unwrap();
        let dns_response = serde_json::from_str::<DNSListResponse>(&dns_response_content);

        if dns_response.is_err() {
            panic!("could not parse response from cloudflare: {:#?}", dns_response.err());
        }

        let cloudflare_dns_list: Vec<DNSRecordSpec> = dns_response.unwrap().result;

        for input_dns in input_cloudflare_dns_list.iter() {
            for dns in cloudflare_dns_list.iter() {
                if input_dns.to_owned() != dns.name {
                    continue;
                }

                let dns_to_update: String = serde_json::to_string(&DNSRecordSpec {
                    id: dns.id.to_owned(),
                    r#type: dns.r#type.to_owned(),
                    name: input_dns.to_owned(),
                    content: current_ip_address.to_owned(),
                    proxied: true,
                })
                .unwrap();

                let dns_update_request = Request::builder()
                    .method("PUT")
                    .uri(format!("{}/{}", cloudflare_api_dns_endpoint, dns.id))
                    .header("Content-Type", "application/json")
                    .header("Authorization", format!("Bearer {}", input_cloudflare_api_token))
                    .body(Body::from(dns_to_update))
                    .expect("request builder to send update request to cloudflare successfully");

                let dns_update_raw_response = client.request(dns_update_request).await?;

                if !dns_update_raw_response.status().is_success() {
                    panic!("failed to update the dns {} records on cloudflare!", input_dns);
                }

                let dns_update_response_content_bytes = hyper::body::to_bytes(dns_update_raw_response).await?;
                let dns_update_response_content = std::str::from_utf8(&dns_update_response_content_bytes).unwrap();
                let dns_update_response: DNSResponse = serde_json::from_str::<DNSResponse>(&dns_update_response_content).unwrap();

                if dns_update_response.success == true {
                    println!("DNS {} was assigned with the following ip {} successfully", dns.name, current_ip_address);
                }

                break;
            }
        }
    }
}