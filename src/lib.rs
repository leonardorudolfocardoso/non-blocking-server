use std::{
    collections::HashMap,
    io::{Error, ErrorKind, Read, Result},
    net::SocketAddr,
    time::Duration,
};

use mio::{net::TcpStream, Events, Interest, Poll, Token};

#[derive(Debug)]
struct Connection {
    stream: TcpStream,
    buf: Vec<u8>,
}

impl Connection {
    fn new(stream: TcpStream) -> Connection {
        Connection {
            stream,
            buf: vec![],
        }
    }

    fn read(&mut self) -> Result<usize> {
        let mut tmp = [0; 1024];
        let n = self.stream.read(&mut tmp)?;

        self.buf.extend_from_slice(&tmp[..n]);
        Ok(n)
    }
}

type Id = usize;
type Connections = HashMap<Id, Connection>;

pub fn serve() -> Result<()> {
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);

    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let mut listener = mio::net::TcpListener::bind(addr)?;

    const SERVER: Token = Token(0);

    poll.registry()
        .register(&mut listener, SERVER, Interest::READABLE)?;
    let mut counter = 1;

    let mut connections = Connections::new();

    loop {
        poll.poll(&mut events, Some(Duration::from_millis(100)))?;

        for event in &events {
            match event.token() {
                SERVER => loop {
                    match listener.accept() {
                        Ok((mut stream, _)) => {
                            let id = counter;
                            counter += 1;
                            poll.registry()
                                .register(&mut stream, Token(id), Interest::READABLE)?;
                            let connection = Connection::new(stream);
                            connections.insert(id, connection);
                        }
                        Err(ref e) if would_block(e) => break,
                        Err(e) => return Err(e),
                    }
                },
                Token(id) => loop {
                    let Some(connection) = connections.get_mut(&id) else {
                        break;
                    };

                    match connection.read() {
                        Ok(ref n) if *n == 0 => {
                            println!("removing client {id}");
                            connections.remove(&id);
                            break;
                        }
                        Ok(_) => {
                            let text = String::from_utf8_lossy(&connection.buf);
                            let text = text.trim();
                            println!("received {text} from client {id}");
                        }
                        Err(ref e) if would_block(e) => break,
                        Err(e) => return Err(e),
                    }
                },
            }
        }
    }
}

fn would_block(e: &Error) -> bool {
    e.kind() == ErrorKind::WouldBlock
}
