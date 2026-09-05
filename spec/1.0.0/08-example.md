[KVD spec](../../README.md), section 08

## 8. Full example

Data (`app.kvd`):

```kvd
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
```kvd

Schema (`app.schema.kvd`): a bare tree whose values are builtin types. The
list-of-strings uses the single-element list form; the open map and list
use the bare `{}`/`[]` leaves:

```kvd
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
