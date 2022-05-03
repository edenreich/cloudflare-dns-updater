## DNS with IPChangeDetector

**DNSRecord** - Two DNS records which would be created on Cloudflare for **example.com** and **admin.example.com**.

**IPChangeDetector** - Runs a check every 1min for the current NAT IP and update the content of DNSRecord[*].metadata.name objects with the current NAT IP.

**AccessToken** - Tells the service how to authenticate with Cloudflare. It references a secret object with the base64 value of the access token generated with the necessary scopes via Cloudflare UI.
