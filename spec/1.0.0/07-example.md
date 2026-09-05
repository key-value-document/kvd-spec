[KVD spec](../../README.md), section 07

## 7. Full example

Data (`app.kvd`):

```
# server config
app:
  name: "hello"
  port: 8080
  debug: true
  when: "2026-08-20"
  greeting: """
    hello
    world
  """
  tags:
    - "web"
    - "api"
  labels: {}
  limits.cpu: 0.5
  endpoints:
    - path: "/health"
      method: "GET"
    - path: "/ready"
      method: "GET"
  dns:
    search: []

tls:
  cert: """
    -----BEGIN CERTIFICATE-----
    MIIB...
    -----END CERTIFICATE-----
  """
```

Schema (`app.schema.kvd`): a bare tree whose values are builtin types:

```
app:
  name: str
  port: int
  debug: bool
  when: str
  greeting: str
  tags:
    - str
  labels: {}
  limits:
    cpu: float
  endpoints:
    - path: str
      method: str
  dns:
    search: []
tls:
  cert: str
```
