## 🔍 `/data` – GET with query parameters

```bash
curl -X GET "http://localhost:8080/data?service=users"
```

> This hits `get_data()` and passes `service=users` as a query param.

---

## 🧪 `/mock` – GET

```bash
curl -X GET "http://localhost:8080/mock"
```

> Calls `mock_get_data()`, which pulls a mock entry from MongoDB by `_id = 10084023`.

---

## 🧠 `/embed` – GET

```bash
curl -X GET "http://localhost:8080/embed"
```

> Calls `mock_get_embed()`, returns a fake embedding vector from OpenAI.

---

## 🧠 `/embed` – POST with path and optional query param

This one expects:

* A `Path<String>` → example: `/embed/mockembed`
* A query param → optional: `?service=auth`
* A raw body (text)

```bash
curl -X POST "http://localhost:8080/embed/mockembed" \
  -H "Content-Type: text/plain" \
  -d "## This is an example text to embed.\n"
```

---

## 🔁 `/api/{*path}` – GET proxy with query param

```bash
curl -X GET "http://localhost:8080/api/status?service=auth"
```

> Proxies to whatever is at `base_url/status` in the `auth` service config.

---

## 🔁 `/api/{*path}` – POST proxy with raw body

```bash
curl -X POST "http://localhost:8080/api/login?service=auth" \
  -H "Content-Type: application/json" \
  -d '{"username": "stephen", "password": "rustacean"}'
```

> Sends a POST request to the `auth` service via your proxy.

---

## ❤️ `/health` – GET

```bash
curl -X GET "http://localhost:8080/health"
```

> Health check endpoint, returns app status and version.

---

### 🚀 TL;DR Table:

| Endpoint            | Method   | Sample `curl`       |
| ------------------- | -------- | ------------------- |
| `/data`             | GET      | `?service=users`    |
| `/mock`             | GET      | —                   |
| `/embed`        | GET      | —                   |
| `/embed/{path}` | POST     | with body           |
| `/api/{*path}`      | GET/POST | proxied via service |
| `/health`           | GET      | —                   |


