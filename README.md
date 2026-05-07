# r_search

A minimal HTTP server written in Rust.

## Usage

```bash
cargo run
```

Server starts on `http://127.0.0.1:8080`.

## Routes

| Method | Path              | Response          |
|--------|-------------------|-------------------|
| GET    | `/health`         | `OK`              |
| GET    | `/echo/<message>` | `<message>`       |
| GET    | `*`               | `404 Not Found`   |

## Features

- TCP-based HTTP server with zero external dependencies
- Routing with health check and echo endpoints
- Error handling for malformed requests (400 Bad Request)
