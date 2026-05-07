use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use rand::Rng;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::{
    models::*,
    state::AppState,
};

fn hash_passkey(passkey: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(passkey.as_bytes());
    hex::encode(hasher.finalize())
}

fn generate_passkey() -> String {
    // 6-digit numeric passkey, easy to read aloud
    let mut rng = rand::thread_rng();
    format!("{:06}", rng.gen_range(100_000..999_999))
}

pub async fn create_meeting(
    State(state): State<Arc<AppState>>,
    Json(req): Json<CreateMeetingRequest>,
) -> Result<Json<CreateMeetingResponse>, StatusCode> {
    let meeting_id = Uuid::new_v4().to_string();
    let host_peer_id = Uuid::new_v4();
    let passkey = generate_passkey();
    let passkey_hash = hash_passkey(&passkey);
    let title = req.title.unwrap_or_else(|| "Untitled Meeting".to_string());
    let now = Utc::now().timestamp();

    let host_token = issue_token(
        &state.jwt_secret,
        host_peer_id,
        &meeting_id,
        "host",
        &req.host_name,
        8,
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query!(
        "INSERT INTO meetings (id, passkey, host_token, title, created_at) VALUES (?, ?, ?, ?, ?)",
        meeting_id,
        passkey_hash,
        host_token,
        title,
        now
    )
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Record host as participant
    let host_id_str = host_peer_id.to_string();
    sqlx::query!(
        "INSERT INTO participants (id, meeting_id, display_name, role, joined_at) VALUES (?, ?, ?, 'host', ?)",
        host_id_str,
        meeting_id,
        req.host_name,
        now
    )
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Create in-memory room
    state.get_or_create_room(&meeting_id, host_peer_id);

    let join_url = format!("/?meeting={}&role=host", meeting_id);

    Ok(Json(CreateMeetingResponse {
        meeting_id: meeting_id.clone(),
        passkey: passkey.clone(), // Shown once to host
        host_token,
        join_url,
    }))
}

pub async fn join_meeting(
    State(state): State<Arc<AppState>>,
    Path(meeting_id): Path<String>,
    Json(req): Json<JoinMeetingRequest>,
) -> Result<Json<JoinMeetingResponse>, StatusCode> {
    // Load meeting from DB
    let meeting = sqlx::query_as!(
        MeetingRow,
        "SELECT * FROM meetings WHERE id = ?",
        meeting_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    // Check closed
    if meeting.closed_at.is_some() {
        return Err(StatusCode::GONE);
    }

    // Verify passkey
    let provided_hash = hash_passkey(&req.passkey);
    if provided_hash != meeting.passkey {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Check 300-participant cap
    let count: i64 = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM participants WHERE meeting_id = ? AND left_at IS NULL",
        meeting_id
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if count >= 300 {
        return Err(StatusCode::PAYLOAD_TOO_LARGE);
    }

    let peer_id = Uuid::new_v4();
    let now = Utc::now().timestamp();
    let peer_id_str = peer_id.to_string();

    let token = issue_token(
        &state.jwt_secret,
        peer_id,
        &meeting_id,
        "participant",
        &req.display_name,
        8,
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query!(
        "INSERT INTO participants (id, meeting_id, display_name, role, joined_at) VALUES (?, ?, ?, 'participant', ?)",
        peer_id_str,
        meeting_id,
        req.display_name,
        now
    )
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(JoinMeetingResponse {
        peer_id: peer_id_str,
        token,
        meeting_id,
        display_name: req.display_name,
        role: "participant".to_string(),
    }))
}

pub async fn get_meeting(
    State(state): State<Arc<AppState>>,
    Path(meeting_id): Path<String>,
) -> Result<Json<MeetingInfo>, StatusCode> {
    let row = sqlx::query_as!(
        MeetingRow,
        "SELECT * FROM meetings WHERE id = ?",
        meeting_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    let count: i64 = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM participants WHERE meeting_id = ? AND left_at IS NULL",
        meeting_id
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(MeetingInfo {
        id: row.id,
        title: row.title,
        participant_count: count,
        created_at: row.created_at,
    }))
}
