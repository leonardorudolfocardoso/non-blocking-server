use non_blocking_server::serve;
use std::{
    env,
    io::{Error, ErrorKind, Result},
    net::SocketAddr,
};

fn main() -> Result<()> {
    let mut args = env::args();
    let _program = args.next();

    let addr: SocketAddr = args
        .next()
        .unwrap_or("127.0.0.1:8080".to_string())
        .parse()
        .map_err(|e| {
            Error::new(
                ErrorKind::InvalidInput,
                format!("invalid socket address: {e}"),
            )
        })?;

    serve(addr)
}
