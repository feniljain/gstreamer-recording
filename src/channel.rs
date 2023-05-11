use futures_channel::mpsc::UnboundedSender;
use futures_util::{future, pin_mut, StreamExt};
use serde_json::json;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};
use url::Url;

use crate::handler::Handler;

pub type SocketTy = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub struct Channel {
    // conn: SocketTy,
    tx: UnboundedSender<Message>,
    handler: Handler,
}

impl Channel {
    pub async fn connect() -> anyhow::Result<Self> {
        let conn_url = Url::parse("ws://localhost:8443/socket")?;

        let (tx, rx) = futures_channel::mpsc::unbounded();
        // tokio::spawn();

        let (ws_stream, _) = connect_async(conn_url).await?;
        println!("Successfully completed websocket handshake");

        let (write, read) = ws_stream.split();

        let stdin_to_ws = rx.map(Ok).forward(write);
        let ws_to_stdout = {
            read.for_each(|message| async {
                let data = message.unwrap().into_text();
                println!("{:?}", data);
            })
        };

        pin_mut!(stdin_to_ws, ws_to_stdout);
        future::select(stdin_to_ws, ws_to_stdout).await;
        Ok(Self {
            tx,
            // conn: ws_stream,
            handler: Handler::new(),
        })
    }

    pub fn send(&self) -> anyhow::Result<()> {
        let msg = json!({
            "type": "welcome"
        });
        self.tx.unbounded_send(Message::Text(msg.to_string()))?;

        Ok(())
    }

    fn recv_msg() {}

    pub async fn close(&mut self) -> anyhow::Result<()> {
        // self.conn.close(None).await?;
        Ok(())
    }
}
