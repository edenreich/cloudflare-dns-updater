## Configure

```sh
kubectl -n kube-system create configmap cloudflare-config --from-literal=zone-id='[ZONE_ID]' --from-literal=dns-list='www.example.com example.com' 
```

Add your access token as a secret:

```sh
kubectl -n kube-system create secret generic cloudflare-credentials --from-literal=access-token='[ACCESS_TOKEN]'
```
