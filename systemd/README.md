## Configure

Download the service file and move it to `/etc/systemd/system/`:

```sh
curl -sSL https://raw.githubusercontent.com/edenreich/cloudflare-dns-updater/master/systemd/cloudflare.service -o ~/cloudflare.service
chmod 644 ~/cloudflare.service
mv ~/cloudflare.service /etc/systemd/system/cloudflare.service
```

Download the env file:

```sh
curl -sSL https://raw.githubusercontent.com/edenreich/cloudflare-dns-updater/master/systemd/cloudflare.env -o ~/cloudflare.env
chmod 400 ~/cloudflare.env
sudo mv ~/cloudflare.env /etc/systemd/system/cloudflare.env
```

Modify the env file and add your access token and zone id:

```sh
sudo vim `/etc/systemd/system/cloudflare.service.env`
```

Finally activate the service, run: 

```sh
sudo systemctl enable cloudflare
sudo systemctl start cloudflare
```