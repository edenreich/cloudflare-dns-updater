FROM alpine:3.11.5

ENV CLOUDFLARE_ZONE_ID=$CLOUDFLARE_ZONE_ID
ENV CLOUDFLARE_ACCESS_TOKEN=$CLOUDFLARE_ACCESS_TOKEN
ENV CLOUDFLARE_DNS_LIST=$CLOUDFLARE_DNS_LIST

RUN apk add --update --no-cache curl

RUN curl -L https://github.com/edenreich/cloudflare-dns-updater/releases/download/v1.0.1/cloudflare_arm -o cloudflare \
    && chmod u+x cloudflare \
    && mv cloudflare /usr/bin/cloudflare

CMD [ "/bin/sh", "-c" "cloudflare update --dns $CLOUDFLARE_DNS_LIST" ]