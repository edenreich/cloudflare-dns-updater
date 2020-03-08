
# Cloudflare-DNS-Updater

Most ISP provide a dynamic public IP address, once your ISP changes the public IP address, this service will pickup on this changes and update your DNS records on cloudflare, to maintain
the IP address statically (P.S this is a FREE solution, due note you could still purchase from some ISP a static IP).

## Usage

Just run `cargo build --release` and use the cloureflare binary like the following:

```sh
CLOUDFLARE_ACCESS_TOKEN=[ACCESS_TOKEN] \
CLOUDFLARE_ZONE_ID=[ZONE_ID] \
cloudflare update \ 
    --dns [DNS_LIST..] \
    --intervals 5
```

Run this ideally controlled by systemd, example service would like like this:

```sh
# path: /etc/systemd/system/cloudflare.service
[Unit]
Description=Cloudflare DNS Updater
Documentation=https://github.com/edenreich/cloudflare-dns-updater
Wants=network-online.target

[Install]
WantedBy=multi-user.target

[Service]
Type=notify
KillMode=process
Delegate=yes
LimitNOFILE=infinity
LimitNPROC=infinity
LimitCORE=infinity
TasksMax=infinity
TimeoutStartSec=0
Restart=always
RestartSec=5s
ExecStartPre=-/sbin/modprobe br_netfilter
ExecStartPre=-/sbin/modprobe overlay
ExecStart=/usr/bin/cloudflare update \ 
    --token=[ACCESS_TOKEN] \
    --zone=[ZONE_ID] \
    --dns=[DNS_LIST..] \
    --intervals=5
```

Finally run: 
```sh
sudo systemctl enable cloudflare
sudo systemctl start cloudflare
```

## Download

You may also download the released binary and simply use it:

```sh
sudo curl -sSL "https://github.com/edenreich/cloudflare-dns-updater/releases/download/v1.0.0/cloudflare" -o /usr/local/bin/cloudflare
sudo chmod +x /usr/local/bin/cloudflare
sudo ln -s /usr/local/bin/cloudflare /usr/bin/cloudflare
```