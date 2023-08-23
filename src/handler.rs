use futures_util::{stream::SplitSink, SinkExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::model::{IncomingMessage, OutgoingMessage, PeerRole, Peer};
use crate::webrtcsrc::WebrtcSrc;

pub struct Handler {
    pub write: SplitSink<SocketTy, Message>,
    self_peer_id: Option<String>,
    producers: Vec<Peer>,
}

pub type SocketTy = WebSocketStream<MaybeTlsStream<TcpStream>>;

impl Handler {
    pub fn new(write: SplitSink<SocketTy, Message>) -> Self {
        Self {
            write,
            self_peer_id: None,
            producers: vec![],
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

                self.self_peer_id = Some(peer_id);

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
                if peer_status.peer_id == self.self_peer_id {
                    let list_msg =
                        serde_json::to_string(&OutgoingMessage::List {
                        peer_id: self.self_peer_id.clone().expect("wao peer id not present"),
                    })?;
                    self.write.send(Message::text(list_msg)).await?;
                }
            }
            IncomingMessage::List { producers } => {
                println!("received all producers: {:?}", producers);
                self.producers = producers;

                let webrtcsrc = WebrtcSrc::new(self.producers[0].id.clone());
                webrtcsrc.launch();
            }
        }

        Ok(())
    }
}
