[KVD spec](../../README.md), section 08

## 7. Full example

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
  labels:
    = "team": "web"
    = "app.kubernetes.io/name": "hello"
  metrics:
    = "a.b.c/name": 99.9
    = "errors/total": 3
  limits.cpu: 0.5
  endpoints:
    - path: "/health"
      method: "GET"
    - path: "/ready"
      method: "GET"
  matrix:
    -
      - 1
      - 2
    -
      - 3
      - 4
  groups:
    = "team-a":
      - "amy"
      - "bo"
  dns:
    search: []

tls:
  cert: """
    -----BEGIN CERTIFICATE-----
    MIIB...
    -----END CERTIFICATE-----
  """
```

Schema (`app.schema.kvd`): a bare tree whose values are builtin types. The
list-of-strings uses the single-element list form; the dicts declare
per-key value types (declared keys may be absent, undeclared data keys pass
unchecked); the open list uses the bare `[]` leaf:

```kvd
app:
  name: str
  port: int
  debug: bool
  when: str
  greeting: str
  tags:
    - str
  labels:
    = "team": str
  metrics:
    = "a.b.c/name": float
    = "errors/total": int
  limits:
    cpu: float
  endpoints:
    - path: str
      method: str
  matrix:
    -
      - int
  groups:
    = "team-a":
      - str
  dns:
    search: []
tls:
  cert: str
```
