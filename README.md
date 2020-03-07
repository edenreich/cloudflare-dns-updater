
# Cloudflare-DNS-Updater

This service update DNS records on Cloudflare with dynamic public IP-Address.

It will check for ip changes every 2 sec by default and update the DNS records on cloudflare once the ip address has been changed.

## Usage

Just run `cargo build --release` and use the cloureflare binary like the following:

cloudflare update --token [ACCESS_TOKEN] --zone [ZONE_ID] --dns [DNS_LIST..] 

Run this ideally controlled by systemd.
