pub mod typings {
    use std::io;
    use thiserror::Error;
    use kube::CustomResource;
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};
    use validator::Validate;

    #[derive(Debug, Error)]
    pub enum Error {
        #[error("There was an error while sending a request to Cloudflare: {0}")]
        CloudflareAPICallError(#[source] io::Error),
        #[error("There was an http error: {0}")]
        HttpError(#[source] io::Error),
    }
    
    #[derive(CustomResource, Clone, Debug, Serialize, Deserialize, Validate, JsonSchema)]
    #[kube(group = "crds.cloudflare.com", version = "v1", kind = "DNSRecord", namespaced)]
    pub struct DNSRecordSpec {
        pub id: String,
        pub r#type: String,
        pub name: String,
        pub content: String,
        pub proxied: bool,
    }
    
    #[derive(Serialize, Deserialize)]
    pub struct DNSResponse {
        pub result: DNSRecordSpec,
        pub success: bool,
        pub errors: Vec<String>,
        pub messages: Vec<String>,
    }
    
    #[derive(Serialize, Deserialize)]
    pub struct DNSListResponse {
        pub result: Vec<DNSRecordSpec>,
        pub success: bool,
        pub errors: Vec<String>,
        pub messages: Vec<String>,
    }
}

pub mod v4 {
    pub mod dns {
        extern crate hyper;
        extern crate hyper_tls;
        extern crate serde;
        extern crate serde_json;

        use crate::typings::{
            DNSResponse,
            DNSRecordSpec,
            DNSListResponse,
            Error,
        };
        use std::{env, io};
        use hyper_tls::HttpsConnector;
        use hyper::{Body, Client, Request};

        fn get_access_token() -> String {
            // get CLOUDFLARE_ACCESS_TOKEN from env (for now just take it from the env afterwards we will use a secret)
            let token = env::var("CLOUDFLARE_ACCESS_TOKEN").expect("CLOUDFLARE_ACCESS_TOKEN must be set");
            token
        }

        pub async fn fetch_record(dns: &DNSRecordSpec) -> Option<&DNSRecordSpec> {
            let https = HttpsConnector::new();
            let client = Client::builder().build::<_, hyper::Body>(https);
        
            let access_token = get_access_token();
        
            let dns_record_request = Request::builder()
                .method("GET")
                .uri(&format!(
                    "https://api.cloudflare.com/client/v4/zones/{}/dns_records",
                    env::var("CLOUDFLARE_ZONE_ID").unwrap()
                ))
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", access_token))
                .body(Body::empty())
                .expect("Request builder to return a dns list");
        
            let dns_raw_response = client.request(dns_record_request).await.unwrap();
        
            if !dns_raw_response.status().is_success() {
                return None;
            }
        
            let dns_response_content_bytes: &hyper::body::Bytes = &hyper::body::to_bytes(dns_raw_response).await.unwrap();
            let dns_response_content = std::str::from_utf8(&dns_response_content_bytes).unwrap();
            let dns_list_response = serde_json::from_str::<DNSListResponse>(&dns_response_content);
        
            match dns_list_response {
                Ok(dns_list_response) => {
                    for dns_record in dns_list_response.result {
                        if dns_record.name == dns.name && dns_record.r#type == dns.r#type {
                            return Some(dns);
                        }
                    }
                }
                Err(e) => {
                    println!("{}", e)
                }
            }
        
            None
        }

        pub async fn update_record(dns: &DNSRecordSpec) -> Result<&DNSRecordSpec, Error> {
            let https = HttpsConnector::new();
            let client = Client::builder().build::<_, hyper::Body>(https);
        
            let access_token = get_access_token();
            let dns_to_update: String = serde_json::to_string(&DNSRecordSpec {
                id: dns.id.to_owned(),
                r#type: dns.r#type.to_owned(),
                name: dns.name.to_owned(),
                content: dns.content.to_owned(),
                proxied: dns.proxied.to_owned(),
            })
            .unwrap();
        
            let dns_record_request = Request::builder()
                .method("PUT")
                .uri(&format!(
                    "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
                    env::var("CLOUDFLARE_ZONE_ID").unwrap(),
                    dns.id
                ))
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", access_token))
                .body(Body::from(dns_to_update))
                .expect("Request builder to return a dns list");
        
            let dns_raw_response = client.request(dns_record_request).await.unwrap();
            if !dns_raw_response.status().is_success() {
                return Err(Error::CloudflareAPICallError(io::Error::new(
                    io::ErrorKind::Other,
                    "Failed to update a DNS record",
                )));
            }
        
            let dns_response_content_bytes: &hyper::body::Bytes = &hyper::body::to_bytes(dns_raw_response).await.unwrap();
            let dns_response_content = std::str::from_utf8(&dns_response_content_bytes).unwrap();
            let dns_response = serde_json::from_str::<DNSResponse>(&dns_response_content);
        
            match dns_response {
                Ok(dns_response) => {
                    if dns_response.success {
                        return Ok(dns);
                    } else {
                        return Err(Error::CloudflareAPICallError(io::Error::new(
                            io::ErrorKind::Other,
                            "Failed to update a DNS record",
                        )));
                    }
                }
                Err(_e) => {
                    return Err(Error::CloudflareAPICallError(io::Error::new(
                        io::ErrorKind::Other,
                        "Failed to update a DNS record",
                    )));
                }
            };
        }

        pub async fn create_record(dns: &DNSRecordSpec) -> Result<&DNSRecordSpec, Error> {
            let https = HttpsConnector::new();
            let client = Client::builder().build::<_, hyper::Body>(https);
        
            let access_token = get_access_token();
            let dns_to_create: String = serde_json::to_string(&DNSRecordSpec {
                id: dns.id.to_owned(),
                r#type: dns.r#type.to_owned(),
                name: dns.name.to_owned(),
                content: dns.content.to_owned(),
                proxied: dns.proxied.to_owned(),
            })
            .unwrap();
        
            let dns_record_request = Request::builder()
                .method("POST")
                .uri(&format!(
                    "https://api.cloudflare.com/client/v4/zones/{}/dns_records",
                    env::var("CLOUDFLARE_ZONE_ID").unwrap()
                ))
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", access_token))
                .body(Body::from(dns_to_create))
                .expect("request builder to return a dns list");
        
            let dns_raw_response = client.request(dns_record_request).await.unwrap();
            if !dns_raw_response.status().is_success() {
                return Err(Error::CloudflareAPICallError(io::Error::new(
                    io::ErrorKind::Other,
                    "Failed to create a DNS record",
                )));
            }
        
            let dns_response_content_bytes: &hyper::body::Bytes = &hyper::body::to_bytes(dns_raw_response).await.unwrap();
            let dns_response_content = std::str::from_utf8(&dns_response_content_bytes).unwrap();
            let dns_response = serde_json::from_str::<DNSResponse>(&dns_response_content);
        
            match dns_response {
                Ok(dns_response) => {
                    if dns_response.success {
                        return Ok(dns);
                    } else {
                        return Err(Error::CloudflareAPICallError(io::Error::new(
                            io::ErrorKind::Other,
                            "Failed to create a DNS record",
                        )));
                    }
                }
                Err(_e) => {
                    return Err(Error::CloudflareAPICallError(io::Error::new(
                        io::ErrorKind::Other,
                        "Failed to create a DNS record",
                    )));
                }
            };
        }
    }
}

pub mod utils {
    use std::io;
    use hyper_tls::HttpsConnector;
    use hyper::{Body, Client, Request};
    use crate::typings::{
        Error,
    };

    pub async fn get_ip_address() -> Result<String, Error> {
        let geoip_api_endpoint: String = "https://checkip.amazonaws.com".to_owned();
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);
        let ip_address_request = Request::builder()
            .method("GET")
            .uri(&geoip_api_endpoint)
            .header("Content-Type", "application/json")
            .body(Body::empty())
            .expect("request builder to return ip address");
    
        let ip_address_raw_response = client.request(ip_address_request).await.unwrap();
    
        if !ip_address_raw_response.status().is_success() {
            return Err(Error::HttpError(io::Error::new(io::ErrorKind::Other, "Failed to get ip address")));
        }
    
        let ip_response_content = hyper::body::to_bytes(ip_address_raw_response).await.unwrap();
    
        Ok(std::str::from_utf8(&ip_response_content).unwrap().to_owned())
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::get_ip_address;

    #[tokio::test]
    async fn test_get_ip_address() {
        let ip_address = match get_ip_address().await {
            Ok(ip_address) => ip_address,
            Err(e) => panic!("failed to get ip address: {}", e),
        };
        println!("{}", ip_address.to_owned());
        assert!(ip_address.len() > 0);
    }
}