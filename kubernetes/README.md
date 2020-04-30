## Configure

```sh
kubectl -n kube-system create configmap cloudflare-config \
    --from-literal=zone-id='[ZONE_ID]' \
    --from-literal=dns-list='www.example.com example.com' \
    --from-literal=ip-check-interval-in-sec='30'
```

Add your access token as a secret:

```sh
kubectl -n kube-system create secret generic cloudflare-credentials \
    --from-literal=access-token='[ACCESS_TOKEN]'
```

## Deploy

To deploy this as a long running system pod on kubernetes:

```sh
kubectl apply -f https://raw.githubusercontent.com/edenreich/cloudflare-dns-updater/master/kubernetes/manifests/linux-arm/cloudflare-dns-updater.yaml
# or
# kubectl apply -f https://raw.githubusercontent.com/edenreich/cloudflare-dns-updater/master/kubernetes/manifests/linux/cloudflare-dns-updater.yaml
```

## Cleanup

```sh
kubectl delete -f https://raw.githubusercontent.com/edenreich/cloudflare-dns-updater/master/kubernetes/manifests/linux-arm/cloudflare-dns-updater.yaml
# or
# kubectl delete -f https://raw.githubusercontent.com/edenreich/cloudflare-dns-updater/master/kubernetes/manifests/linux/cloudflare-dns-updater.yaml

kubectl -n kube-system delete configmaps/cloudflare-config
kubectl -n kube-system delete secrets/cloudflare-credentials
```