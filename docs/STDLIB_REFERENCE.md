# Standard Library Reference

## Runtime (`runtime/`)

### Core (`core.aether`)
Basic runtime functions including memory management and I/O.

### Vector (`vec.aether`)
Dynamic array implementation.

```aether
let v = vec_new()
vec_push(v, 42)
let val = vec_get(v, 0)
```

### HashMap (`map.aether`)
Hash-based key-value storage.

```aether
let m = map_new()
map_set(m, key, value)
let val = map_get(m, key)
```

### String (`str.aether`)
String manipulation functions.

```aether
let len = str_len(s)
let eq = str_eq(s1, s2)
```

## Networking (`runtime/`)

### TCP/UDP (`net.aether`)
Low-level socket operations.

```aether
let fd = tcp_socket()
tcp_connect_ip(fd, 127, 0, 0, 1, 8080)
tcp_send(fd, data, len)
tcp_recv(fd, buf, max_len)
tcp_close(fd)
```

### HTTP (`http.aether`)
HTTP client for REST APIs.

```aether
let req = http_request_new(HTTP_GET, host, path)
let res = http_execute_ip(req, ip_a, ip_b, ip_c, ip_d, 80)
let status = http_response_status(res)
let body = http_response_body(res)
```

### DNS (`dns.aether`)
Hostname resolution.

```aether
let result = dns_resolve(hostname)
if dns_result_success(result) {
    let ip_a = dns_result_ip_a(result)
    // ...
}
```

### TLS (`tls.aether`)
Secure connections with TLS 1.2.

```aether
let tls = https_connect(ip_a, ip_b, ip_c, ip_d)
tls_send(tls, data, len)
tls_recv(tls, buf, max_len)
tls_close(tls)
```

## Cryptography (`runtime/crypto/`)

### SHA-256 (`sha256.aether`)
Cryptographic hashing.

```aether
let hash = sha256(data, len)
let hash = sha256_str(string)
```

### AES-GCM (`aes_gcm.aether`)
Authenticated encryption.

```aether
aes_gcm_encrypt(key, key_len, iv, plaintext, pt_len, ciphertext, tag)
```

## Firebase (`stdlib/firebase/`)

### Core (`firebase.aether`)
Firebase SDK initialization and auth.

```aether
let config = firebase_config_new(project_id, api_key)
let app = firebase_init(config)
let auth = firebase_auth(app)
```

### Data Connect (`dataconnect.aether`)
GraphQL-based data access.

```aether
let dc = dataconnect_new(project, location, service, connector)
let result = dataconnect_query(dc, query)
```

### App Hosting (`app_hosting.aether`)
Server-side deployment.

```aether
let ah = apphosting_new(project_id, site_id)
let deploy = apphosting_deploy(ah, source_dir)
```

## Database (`stdlib/database/`)

### PostgreSQL (`postgres.aether`)
Real wire protocol implementation.

```aether
let conn = pg_connect(ip_a, ip_b, ip_c, ip_d, 5432, database, user, password)
let result = pg_query(conn, "SELECT * FROM users")
let rows = pg_result_rows(result)
pg_close(conn)
```

## Cloud (`stdlib/cloud/`)

### Cloud SQL (`cloudsql.aether`)
Google Cloud SQL connectivity.

```aether
let cs = cloudsql_new(project, region, instance)
cloudsql_set_credentials(cs, database, user, password)
cloudsql_connect_ip(cs, ip_a, ip_b, ip_c, ip_d, 5432)
```

### Cloud Run (`cloudrun.aether`)
Container deployment.

```aether
let svc = cloudrun_service_new(project, region, name)
cloudrun_set_image(svc, image)
cloudrun_deploy(svc, auth_token)
```
