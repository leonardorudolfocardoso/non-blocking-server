# Non-Blocking Server

A small Rust TCP server that uses [`mio`](https://docs.rs/mio/) to handle
socket readiness with an explicit polling loop.

By default, the server listens on `127.0.0.1:8080`, accepts TCP clients, reads
incoming bytes into a per-connection buffer, and logs the received text to
stdout.

## Run

Start the server:

```sh
cargo run
```

By default, the server listens on `127.0.0.1:8080`. To bind a different
address, pass it as the first argument:

```sh
cargo run -- 127.0.0.1:9000
```

The process stays running and polls for socket events until it is interrupted.

## Manual Test

In another terminal, connect with `nc`:

```sh
nc 127.0.0.1 8080
```

Type a message and press Enter. The server logs the accumulated text for that
client connection.

## Architecture

- `src/main.rs` contains the binary entrypoint and delegates to `serve()` or
  `serve_addr()`.
- `src/lib.rs` contains the TCP server implementation.
- `serve()` starts the server with the default bind address.
- `serve_addr()` creates a `mio::Poll`, registers the TCP listener, accepts
  readable client sockets, and registers each client with its own token.
- Active clients are stored in a `HashMap` keyed by connection id.
- Each `Connection` owns its `TcpStream` and an accumulated read buffer.

## Current Limitations

- The server only reads and logs data; it does not send responses.
- Connection buffers grow for the lifetime of the connection.
- There is no application-level protocol or message framing.
- Client streams are not explicitly deregistered before removal.
