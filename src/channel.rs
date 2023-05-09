use std::net::TcpStream;

use tungstenite::{connect, stream::MaybeTlsStream, WebSocket};
use url::Url;

use crate::handler::Handler;

pub type SocketTy = WebSocket<MaybeTlsStream<TcpStream>>;

pub struct Channel {
    conn: SocketTy,
    handler: Handler,
}

impl Channel {
    pub fn connect() -> Self {
        let (socket, response) =
            connect(Url::parse("ws://localhost:8443/socket").unwrap()).expect("Can't connect");

        println!("Connected to the server");
        println!("Response HTTP code: {}", response.status());
        println!("Response contains the following headers:");

        for (ref header, _value) in response.headers() {
            println!("* {}", header);
        }

        socket.

        Self {
            conn: socket,
            handler: Handler::new(),
        }
    }

    pub fn close(&mut self) -> anyhow::Result<()> {
        self.conn.close(None)?;
        Ok(())
    }
}
