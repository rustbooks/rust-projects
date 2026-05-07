use std::sync::Arc;

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, Query, State,
    },
    response::IntoResponse,
};
use serde::Deserialize;
use tokio::sync::broadcast;
use tracing::{info, warn};
use uuid::Uuid;

use crate::{
    models::*,
    state::{AppState, Peer},
};

#[derive(Deserialize)]
pub struct WsQuery {
    pub token: String,
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    Path((meeting_id, peer_id_str)): Path<(String, String)>,
    Query(query): Query<WsQuery>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state, meeting_id, peer_id_str, query.token))
}

async fn handle_socket(
    mut socket: WebSocket,
    state: Arc<AppState>,
    meeting_id: String,
    peer_id_str: String,
    token: String,
) {
    // Verify JWT
    let claims = match verify_token(&state.jwt_secret, &token) {
        Ok(c) => c,
        Err(_) => {
            let msg = serde_json::to_string(&WsServerMsg::Error {
                message: "Invalid token".to_string(),
            })
            .unwrap();
            let _ = socket.send(Message::Text(msg)).await;
            return;
        }
    };

    let peer_id = match Uuid::parse_str(&peer_id_str) {
        Ok(id) => id,
        Err(_) => return,
    };

    let host_id: Uuid = {
        let row = sqlx::query_scalar!(
            "SELECT id FROM participants WHERE meeting_id = ? AND role = 'host' LIMIT 1",
            meeting_id
        )
        .fetch_optional(&state.db)
        .await
        .unwrap_or(None)
        .unwrap_or_default();
        Uuid::parse_str(&row).unwrap_or(Uuid::nil())
    };

    let room = state.get_or_create_room(&meeting_id, host_id);

    // Per-peer broadcast channel
    let (peer_tx, _) = broadcast::channel::<String>(256);
    let peer = Peer {
        id: peer_id,
        display_name: claims.name.clone(),
        role: claims.role.clone(),
        is_muted: false,
        is_video_off: false,
        tx: peer_tx.clone(),
    };
    room.peers.insert(peer_id, peer);

    // Subscribe to room-wide events
    let mut room_rx = room.room_tx.subscribe();
    let mut peer_rx = peer_tx.subscribe();

    info!("Peer {} ({}) joined meeting {}", peer_id, claims.name, meeting_id);

    // Send welcome
    let welcome = serde_json::to_string(&WsServerMsg::Welcome {
        peer_id: peer_id.to_string(),
        meeting_id: meeting_id.clone(),
    })
    .unwrap();
    if socket.send(Message::Text(welcome)).await.is_err() {
        return;
    }

    // Broadcast participant-joined to room
    let join_msg = serde_json::to_string(&WsServerMsg::ParticipantJoined {
        peer_id: peer_id.to_string(),
        display_name: claims.name.clone(),
        role: claims.role.clone(),
    })
    .unwrap();
    let _ = room.room_tx.send(join_msg);

    // Send current participant list to new joiner
    let participants: Vec<ParticipantInfo> = room
        .peers
        .iter()
        .map(|p| ParticipantInfo {
            peer_id: p.id.to_string(),
            display_name: p.display_name.clone(),
            role: p.role.clone(),
            is_muted: p.is_muted,
        })
        .collect();
    let list_msg = serde_json::to_string(&WsServerMsg::ParticipantList { participants }).unwrap();
    let _ = socket.send(Message::Text(list_msg)).await;

    // Main loop: fan-in from WS client + room broadcasts + peer events
    loop {
        tokio::select! {
            // Incoming message from this client
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        if let Err(e) = handle_client_message(&text, peer_id, &claims, &room, &state).await {
                            warn!("Error handling WS message: {e}");
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Ok(Message::Ping(data))) => {
                        let _ = socket.send(Message::Pong(data)).await;
                    }
                    _ => {}
                }
            }

            // Room-wide broadcast (chat, participant events)
            Ok(msg) = room_rx.recv() => {
                if socket.send(Message::Text(msg)).await.is_err() {
                    break;
                }
            }

            // Peer-specific event (mute, kick)
            Ok(msg) = peer_rx.recv() => {
                if socket.send(Message::Text(msg)).await.is_err() {
                    break;
                }
            }
        }
    }

    // Clean up on disconnect
    room.peers.remove(&peer_id);
    let left_msg = serde_json::to_string(&WsServerMsg::ParticipantLeft {
        peer_id: peer_id.to_string(),
    })
    .unwrap();
    let _ = room.room_tx.send(left_msg);

    let now = chrono::Utc::now().timestamp();
    let _ = sqlx::query!(
        "UPDATE participants SET left_at = ? WHERE id = ? AND meeting_id = ?",
        now,
        peer_id_str,
        meeting_id
    )
    .execute(&state.db)
    .await;

    info!("Peer {} left meeting {}", peer_id, meeting_id);
}

async fn handle_client_message(
    text: &str,
    sender_id: Uuid,
    claims: &Claims,
    room: &crate::state::Room,
    state: &AppState,
) -> anyhow::Result<()> {
    let msg: WsClientMsg = serde_json::from_str(text)?;

    match msg {
        WsClientMsg::Chat { text, to } => {
            let chat = WsServerMsg::Chat {
                from: sender_id.to_string(),
                from_name: claims.name.clone(),
                text,
                private: to.is_some(),
            };
            let payload = serde_json::to_string(&chat)?;

            if let Some(target_id_str) = to {
                // Private message: send only to target peer
                if let Ok(target_id) = Uuid::parse_str(&target_id_str) {
                    if let Some(peer) = room.peers.get(&target_id) {
                        let _ = peer.tx.send(payload);
                    }
                }
            } else {
                // Broadcast to all
                let _ = room.room_tx.send(payload);
            }
        }

        // SDP / ICE: relay to all peers (clients do peer-to-SFU via WHIP, but
        // some p2p fallback signaling still goes through here)
        WsClientMsg::SdpOffer { sdp } => {
            let relay = WsServerMsg::SdpOffer {
                from: sender_id.to_string(),
                sdp,
            };
            let _ = room.room_tx.send(serde_json::to_string(&relay)?);
        }

        WsClientMsg::SdpAnswer { sdp } => {
            let relay = WsServerMsg::SdpAnswer {
                from: sender_id.to_string(),
                sdp,
            };
            let _ = room.room_tx.send(serde_json::to_string(&relay)?);
        }

        WsClientMsg::IceCandidate { candidate, sdp_mid, sdp_m_line_index } => {
            let relay = WsServerMsg::IceCandidate {
                from: sender_id.to_string(),
                candidate,
                sdp_mid,
                sdp_m_line_index,
            };
            let _ = room.room_tx.send(serde_json::to_string(&relay)?);
        }

        WsClientMsg::Ping => {
            if let Some(peer) = room.peers.get(&sender_id) {
                let _ = peer.tx.send(serde_json::to_string(&WsServerMsg::Pong)?);
            }
        }
    }

    Ok(())
}
