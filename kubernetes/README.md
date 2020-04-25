## Configure

```sh
kubectl -n kube-system create configmap cloudflare-config --from-literal=zone-id='[ZONE_ID]' --from-literal=dns-list='www.example.com example.com' 
```

Add your access token as a secret:

```sh
kubectl -n kube-system create secret generic cloudflare-credentials --from-literal=access-token='[ACCESS_TOKEN]'
```

## Deploy

To deploy this as a long running system pod on kubernetes:

```sh
kubectl apply -f https://raw.githubusercontent.com/edenreich/cloudflare-dns-updater/master/kubernetes/manifests/linux-arm/cloudflare-dns-updater.yaml
# or
# kubectl apply -f https://raw.githubusercontent.com/edenreich/cloudflare-dns-updater/master/kubernetes/manifests/linux/cloudflare-dns-updater.yaml
```