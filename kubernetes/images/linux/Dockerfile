FROM alpine:3.11.5

RUN apk add --update --no-cache curl

RUN curl -L https://github.com/edenreich/cloudflare-dns-updater/releases/download/v1.0.2/cloudflare -o cloudflare \
    && chmod u+x cloudflare \
    && mv cloudflare /usr/bin/cloudflare

CMD [ "/bin/sh", "-c", "cloudflare update --dns $CLOUDFLARE_DNS_LIST --intervals $IP_CHECK_INTERVAL_IN_SEC" ]