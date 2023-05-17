use crate::model::{IncomingMessage, OutgoingMessage, PeerRole};
use futures_util::{stream::SplitSink, SinkExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

pub struct Handler {
    pub write: SplitSink<SocketTy, Message>,
    peer_id: Option<String>,
}

pub type SocketTy = WebSocketStream<MaybeTlsStream<TcpStream>>;

impl Handler {
    pub fn new(write: SplitSink<SocketTy, Message>) -> Self {
        Self {
            write,
            peer_id: None,
        }
    }

    pub async fn handle(&mut self, msg_str: String) -> anyhow::Result<()> {
        println!("received message from signalling server, str: {}", msg_str);

        // gstreamer signalling server send empty string as ping regularly
        // don't try to parse them
        if msg_str == "" {
            return Ok(());
        }

        let incoming_msg: IncomingMessage =
            serde_json::from_str(&msg_str).expect("couldn't desearlize message");

        match incoming_msg {
            IncomingMessage::Welcome { peer_id } => {
                println!("incoming_message: welcome: peer_id: {}", peer_id);

                self.peer_id = Some(peer_id);

                let set_peer_status_msg = serde_json::to_string(&OutgoingMessage::SetPeerStatus {
                    roles: vec![PeerRole::Listener],
                    meta: None,
                })?;
                self.write.send(Message::text(set_peer_status_msg)).await?;
            }
            IncomingMessage::StartSession {
                peer_id,
                session_id,
            } => {
                println!(
                    "incoming_message: start session: peer_id: {} - session_id: {}",
                    peer_id, session_id
                );
            }
            IncomingMessage::SessionStarted {
                peer_id,
                session_id,
            } => {
                println!(
                    "incoming_message: session started: peer_id: {} - session_id: {}",
                    peer_id, session_id
                );
            }
            IncomingMessage::Error { details } => {
                println!("incoming_message: error: details: {}", details);
            }
            IncomingMessage::PeerStatusChanged(peer_status) => {
                println!("peer status changed: peer_status: {:?}", peer_status);
                if peer_status.peer_id == self.peer_id {
                    let list_msg =
                        serde_json::to_string(&OutgoingMessage::List {
                        peer_id: self.peer_id.clone().expect("wao peer id not present"),
                    })?;
                    self.write.send(Message::text(list_msg)).await?;
                }
            }
            IncomingMessage::List { producers } => {
                println!("received all producers: {:?}", producers);
            }
        }

        Ok(())
    }
}
