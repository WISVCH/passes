# passes

Wallet passes integration

## Convery key to legacy

Make sure to convert the apple key to AES-256-CBC:
https://stackoverflow.com/a/72600724/4858692

```sh
# in 3.x.x
openssl pkcs12 -in old -nodes -provider legacy -provider default -out temp && openssl pkcs12 -in temp -export -out new
# or simpler
openssl pkcs12 -in old -nodes -legacy -out temp && openssl pkcs12 -in temp -export -out new

# in 1.x.x
openssl pkcs12 -in old -nodes -out temp && openssl pkcs12 -in temp -export -descert -out new
```
