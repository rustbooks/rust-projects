use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use serde_json::json;
use uuid::Uuid;

use crate::{
    models::{verify_token, WsServerMsg},
    state::AppState,
};

/// Extract and verify JWT from Authorization header, enforce host role
fn require_host(state: &AppState, headers: &HeaderMap, meeting_id: &str) -> Result<String, StatusCode> {
    let auth = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let claims = verify_token(&state.jwt_secret, auth)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    if claims.role != "host" {
        return Err(StatusCode::FORBIDDEN);
    }
    if claims.mid != meeting_id {
        return Err(StatusCode::FORBIDDEN);
    }
    Ok(claims.sub)
}

pub async fn mute_peer(
    State(state): State<Arc<AppState>>,
    Path((meeting_id, peer_id)): Path<(String, String)>,
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, StatusCode> {
    require_host(&state, &headers, &meeting_id)?;

    let pid = Uuid::parse_str(&peer_id).map_err(|_| StatusCode::BAD_REQUEST)?;

    if let Some(room) = state.rooms.get(&meeting_id) {
        if let Some(mut peer) = room.peers.get_mut(&pid) {
            peer.is_muted = true;
            let msg = serde_json::to_string(&WsServerMsg::Muted { peer_id: peer_id.clone() }).unwrap();
            let _ = peer.tx.send(msg.clone());
            let _ = room.room_tx.send(msg);
        }
    }

    Ok(Json(json!({ "ok": true })))
}

pub async fn unmute_peer(
    State(state): State<Arc<AppState>>,
    Path((meeting_id, peer_id)): Path<(String, String)>,
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, StatusCode> {
    require_host(&state, &headers, &meeting_id)?;

    let pid = Uuid::parse_str(&peer_id).map_err(|_| StatusCode::BAD_REQUEST)?;

    if let Some(room) = state.rooms.get(&meeting_id) {
        if let Some(mut peer) = room.peers.get_mut(&pid) {
            peer.is_muted = false;
            let msg = serde_json::to_string(&WsServerMsg::Unmuted { peer_id: peer_id.clone() }).unwrap();
            let _ = peer.tx.send(msg.clone());
            let _ = room.room_tx.send(msg);
        }
    }

    Ok(Json(json!({ "ok": true })))
}

pub async fn kick_peer(
    State(state): State<Arc<AppState>>,
    Path((meeting_id, peer_id)): Path<(String, String)>,
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, StatusCode> {
    require_host(&state, &headers, &meeting_id)?;

    let pid = Uuid::parse_str(&peer_id).map_err(|_| StatusCode::BAD_REQUEST)?;

    if let Some(room) = state.rooms.get(&meeting_id) {
        let msg = serde_json::to_string(&WsServerMsg::Kicked { peer_id: peer_id.clone() }).unwrap();
        if let Some(peer) = room.peers.get(&pid) {
            let _ = peer.tx.send(msg.clone()); // Tell the peer they're kicked
        }
        room.peers.remove(&pid);
        let left_msg = serde_json::to_string(&WsServerMsg::ParticipantLeft { peer_id: peer_id.clone() }).unwrap();
        let _ = room.room_tx.send(left_msg);
    }

    // Mark in DB
    let now = chrono::Utc::now().timestamp();
    let _ = sqlx::query!(
        "UPDATE participants SET left_at = ? WHERE id = ? AND meeting_id = ?",
        now,
        peer_id,
        meeting_id
    )
    .execute(&state.db)
    .await;

    Ok(Json(json!({ "ok": true })))
}

pub async fn promote_peer(
    State(state): State<Arc<AppState>>,
    Path((meeting_id, peer_id)): Path<(String, String)>,
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, StatusCode> {
    require_host(&state, &headers, &meeting_id)?;

    let pid = Uuid::parse_str(&peer_id).map_err(|_| StatusCode::BAD_REQUEST)?;

    if let Some(room) = state.rooms.get(&meeting_id) {
        if let Some(mut peer) = room.peers.get_mut(&pid) {
            peer.role = "host".to_string();
            let msg = serde_json::to_string(&WsServerMsg::Promoted { peer_id: peer_id.clone() }).unwrap();
            let _ = room.room_tx.send(msg);
        }
    }

    Ok(Json(json!({ "ok": true })))
}

pub async fn close_meeting(
    State(state): State<Arc<AppState>>,
    Path(meeting_id): Path<String>,
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, StatusCode> {
    require_host(&state, &headers, &meeting_id)?;

    let now = chrono::Utc::now().timestamp();
    sqlx::query!(
        "UPDATE meetings SET closed_at = ? WHERE id = ?",
        now,
        meeting_id
    )
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Broadcast close to all peers
    if let Some(room) = state.rooms.get(&meeting_id) {
        let msg = serde_json::to_string(&WsServerMsg::Error {
            message: "Meeting closed by host".to_string(),
        })
        .unwrap();
        let _ = room.room_tx.send(msg);
    }
    state.rooms.remove(&meeting_id);

    Ok(Json(json!({ "ok": true })))
}
