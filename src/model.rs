use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Peer {
    pub id: String,
    meta: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum PeerRole {
    Listener,
    Producer,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PeerStatus {
    pub roles: Vec<PeerRole>,
    pub meta: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub peer_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum IncomingMessage {
    /// Welcoming message, sets the Peer ID linked to a new connection
    #[serde(rename_all = "camelCase")]
    Welcome { peer_id: String },
    /// Notifies listeners that a peer status has changed
    PeerStatusChanged(PeerStatus),
    /// Instructs a peer to generate an offer and inform about the session ID
    #[serde(rename_all = "camelCase")]
    StartSession { peer_id: String, session_id: String },
    /// Let consumer know that the requested session is starting with the specified identifier
    #[serde(rename_all = "camelCase")]
    SessionStarted { peer_id: String, session_id: String },
    // /// Signals that the session the peer was in was ended
    // EndSession(EndSessionMessage),
    // /// Messages directly forwarded from one peer to another
    // Peer(PeerMessage),
    #[serde(rename_all = "camelCase")]
    List { producers: Vec<Peer> },
    /// Notifies that an error occurred with the peer's current session
    #[serde(rename_all = "camelCase")]
    Error { details: String },
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum OutgoingMessage {
    #[serde(rename_all = "camelCase")]
    NewPeer,

    #[serde(rename_all = "camelCase")]
    SetPeerStatus {
        roles: Vec<PeerRole>,
        meta: Option<serde_json::Value>,
    },

    #[serde(rename_all = "camelCase")]
    List { peer_id: String },
}
