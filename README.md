
# Cloudflare-DNS-Updater

Most ISP provide a dynamic public IP address, once your ISP changes the public IP address, this service will pickup on this changes and update your DNS records on cloudflare, to maintain
the IP address statically (P.S this is a FREE solution, due note you could still purchase from some ISP a static IP).

## Usage

Just run `cargo build --release` and use the cloureflare binary like the following:

cloudflare update --token [ACCESS_TOKEN] --zone [ZONE_ID] --dns [DNS_LIST..] 

Run this ideally controlled by systemd.

## Download

You may also download the released binary and simply use it.