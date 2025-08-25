## Auto TLS with Traefik & Let's Encrypt

To set up automatic TLS with Traefik and Let's Encrypt:

Traefik is the default ingress controller in k3s. To enable automatic TLS, override Traefik's configuration by adding the following file (k3s will automatically detect and apply changes):

```yaml
# filepath: /var/lib/rancher/k3s/server/manifests/traefik-config.yaml
apiVersion: helm.cattle.io/v1
kind: HelmChartConfig
metadata:
  name: traefik
  namespace: kube-system
spec:
  valuesContent: |-
    additionalArguments:
      - "--certificatesresolvers.default.acme.storage=/data/acme.json"
      - "--certificatesresolvers.default.acme.httpchallenge.entrypoint=web"
      - "--certificatesresolvers.default.acme.email=<your-email>"
    ports:
      web:
        exposedPort: 80
      websecure:
        exposedPort: 443
```

Then, in your Ingress, use the default certresolver and TLS like so:

```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: my-ingress
  namespace: proj
  annotations:
    kubernetes.io/ingress.class: "traefik"
    traefik.ingress.kubernetes.io/router.entrypoints: websecure
    traefik.ingress.kubernetes.io/router.tls.certresolver: default
spec:
  tls:
    - hosts:
        - <your-domain>
  rules:
    - host: <your-domain>
      http:
        paths:
          - path: /
            pathType: Prefix
            backend:
              service:
                name: my-backend
                port:
                  number: 80
```

You can watch Traefik logs for messages like these (or errors if something went wrong):

```log
INF Testing certificate renew... acmeCA=...
INF Starting provider *acme.ChallengeTLSALPN
INF Register... providerName=default.acme
```

That should be all.  
If something doesn’t work, check the Traefik logs, verify your firewall rules, and ensure your DNS A and AAAA records are correct.  
Be aware of Let’s Encrypt’s request limits. You can use `kubectl rollout restart' to force Traefik to retry certificate
