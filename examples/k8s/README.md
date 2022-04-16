## DNS with IPChangeDetector

**DNSRecord** - Two DNS records which would be created on Cloudflare for **example.com** and **admin.example.com**.

**IPChangeDetector** - Will run a check every 1min for the current NAT IP and update the content of DNSRecord[*].metadata.name objects with the current NAT IP.
