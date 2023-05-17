use futures_util::StreamExt;
use tokio::task::JoinHandle;
use tokio_tungstenite::connect_async;
use url::Url;

use crate::handler::Handler;

pub struct Channel {
    pub join_handle: JoinHandle<()>,
    // conn: SocketTy,
    // tx: UnboundedSender<Message>,
    // write: SplitSink<SocketTy, Message>,
}

impl Channel {
    pub async fn connect() -> anyhow::Result<Self> {
        let conn_url = Url::parse("ws://localhost:8443/socket")?;

        // let (tx, rx) = futures_channel::mpsc::unbounded();
        // tokio::spawn();

        let (ws_stream, _) = connect_async(conn_url).await?;
        println!("Successfully completed websocket handshake");

        let (write, mut read) = ws_stream.split();

//         // Send new peer message
//         let new_peer_msg = serde_json::to_string(&OutgoingMessage::NewPeer)?;
//         write.send(Message::text(new_peer_msg)).await?;

        let mut handler = Handler::new(write);

        let join_handle = tokio::spawn(async move {
            loop {
                let message_result = read.next().await.expect("expected message");
                match message_result {
                    Ok(msg) => {
                        let data_result = msg.into_text();
                        match data_result {
                            Ok(data) => handler
                                .handle(data.clone())
                                .await
                                .expect(&format!("error hanlding message: {:?}", data)),
                            Err(err) => panic!("{}", err),
                        }
                    }
                    Err(err) => panic!("{}", err),
                }
            }
        });

        // let stdin_to_ws = rx.map(Ok).forward(write);
        // let ws_to_stdout = {};

        // pin_mut!(stdin_to_ws, ws_to_stdout);
        // future::select(stdin_to_ws, ws_to_stdout).await;
        Ok(Self {
            join_handle,
            // tx,
            // conn: ws_stream,
            // write,
        })
    }

    // pub async fn send(&mut self, outgoing_msg: OutgoingMessage) -> anyhow::Result<()> {
    //     // self.tx.unbounded_send(Message::Text(msg.to_string()))?;

    //     Ok(())
    // }

    // pub async fn close(&mut self) -> anyhow::Result<()> {
    //     // self.conn.close(None).await?;
    //     Ok(())
    // }
}

// async fn recv_msg(read: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>) {}
