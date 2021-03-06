![Build - Linux on ARM (Raspbian Buster)](https://github.com/edenreich/cloudflare-dns-updater/workflows/Build%20-%20Linux%20on%20ARM%20(Raspbian%20Buster)/badge.svg?branch=master)
![Build - Linux (Ubuntu)](https://github.com/edenreich/cloudflare-dns-updater/workflows/Build%20-%20Linux%20(Ubuntu)/badge.svg?branch=master)

# Cloudflare-DNS-Updater

Most ISP provide a dynamic public IP address, once your ISP changes the public IP address, this service will pickup on this changes and update your DNS records on cloudflare, to maintain
the IP address statically (P.S this is a FREE solution, do note that you could still purchase from some ISP a static IP).

- [Cloudflare-DNS-Updater](#Cloudflare-DNS-Updater)
    - [Usage](#Usage)
    - [Build](#Build)
    - [Tests](#Tests)
    - [Download](#Download)
    - [Target](#Target)
    - [Motivation](#Motivation)

## Usage

```sh
CLOUDFLARE_ACCESS_TOKEN=[ACCESS_TOKEN] \
CLOUDFLARE_ZONE_ID=[ZONE_ID] \
cloudflare update \ 
    --dns [DNS_LIST..] \
    --intervals 5
```

Run this as a long running process:
- as a [systemd service](systemd/README.md).
- as a system pod on [kubernetes](kubernetes/README.md) 

## Build

Note: compilation is done using statically linking, to make sure everything comes with the binary as is.

Build on ubuntu:

```sh
docker build -t cloudflare/ubuntu-1910 -f build/ubuntu-1910/Dockerfile .
```

Build on raspberry:

```sh
docker build -t cloudflare/raspbian-buster-20180926 -f build/raspbian-buster-20180926/Dockerfile .
```

Inside the containers binaries are located at `/home/rust/app/bin` directory after complete build.
To use them just copy them out of the containers, for example:

```sh
id=$(docker create --name cloudflare_ubuntu-1910 cloudflare/ubuntu-1910) && \
docker cp cloudflare_ubuntu-1910:/home/rust/app/bin/cloudflare bin/cloudflare && \
docker rm $id

id=$(docker create --name cloudflare_raspbian-buster-20180926 cloudflare/raspbian-buster-20180926) && \
docker cp cloudflare_raspbian-buster-20180926:/home/rust/app/bin/cloudflare bin/cloudflare && \
docker rm $id
```

## Tests

After building and copying the binaries outside of the containers, test to check if it works on:

- ubuntu:
```sh
docker build -t cloudflare/test-ubuntu-1910 -f tests/ubuntu-1910/Dockerfile .
docker run --rm -it cloudflare/test-ubuntu-1910
```

- raspberry:
```sh
docker build -t cloudflare/test-raspbian-buster-20180926 -f tests/raspbian-buster-20180926/Dockerfile .
docker run --rm -it cloudflare/test-raspbian-buster-20180926
```

## Download

You may also download the released binary and simply use it:

```sh
sudo curl -sSL "https://github.com/edenreich/cloudflare-dns-updater/releases/download/v1.0.2/cloudflare" -o /usr/local/bin/cloudflare
# or for ARM:
# sudo curl -sSL "https://github.com/edenreich/cloudflare-dns-updater/releases/download/v1.0.2/cloudflare_arm" -o /usr/local/bin/cloudflare

sudo chmod +x /usr/local/bin/cloudflare
sudo ln -s /usr/local/bin/cloudflare /usr/bin/cloudflare
```

## Target

This project targets linux, works on ARM as well.

## Motivation

I have a raspberry pi k3s cluster at home, and I often find it annoying that the internet provider changes the IP address.
So I thought it could be nice to have a webserver without needing to worry about the IP, that would automatically update my DNS
records on cloudflare to point to that newly dynamic IP provided by my ISP.

## Bugs Reporting

If you find any bug please submit an issue ;)