use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ── DB row types ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct MeetingRow {
    pub id: String,
    pub passkey: String,
    pub host_token: String,
    pub title: String,
    pub created_at: i64,
    pub closed_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ParticipantRow {
    pub id: String,
    pub meeting_id: String,
    pub display_name: String,
    pub role: String,
    pub joined_at: i64,
    pub left_at: Option<i64>,
}

// ── JWT ───────────────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,     // peer_id (UUID string)
    pub mid: String,     // meeting_id
    pub role: String,    // "host" | "participant"
    pub name: String,    // display name
    pub exp: i64,        // expiry unix timestamp
}

pub fn issue_token(
    secret: &str,
    peer_id: Uuid,
    meeting_id: &str,
    role: &str,
    display_name: &str,
    ttl_hours: i64,
) -> anyhow::Result<String> {
    let claims = Claims {
        sub: peer_id.to_string(),
        mid: meeting_id.to_string(),
        role: role.to_string(),
        name: display_name.to_string(),
        exp: Utc::now().timestamp() + ttl_hours * 3600,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;
    Ok(token)
}

pub fn verify_token(secret: &str, token: &str) -> anyhow::Result<Claims> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(data.claims)
}

// ── API request / response shapes ────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreateMeetingRequest {
    pub title: Option<String>,
    pub host_name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateMeetingResponse {
    pub meeting_id: String,
    pub passkey: String,      // plaintext, shown once to the host
    pub host_token: String,
    pub join_url: String,
}

#[derive(Debug, Deserialize)]
pub struct JoinMeetingRequest {
    pub passkey: String,
    pub display_name: String,
}

#[derive(Debug, Serialize)]
pub struct JoinMeetingResponse {
    pub peer_id: String,
    pub token: String,
    pub meeting_id: String,
    pub display_name: String,
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct MeetingInfo {
    pub id: String,
    pub title: String,
    pub participant_count: i64,
    pub created_at: i64,
}

// ── WebSocket message envelopes ───────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WsClientMsg {
    SdpOffer { sdp: String },
    SdpAnswer { sdp: String },
    IceCandidate { candidate: String, sdp_mid: String, sdp_m_line_index: u32 },
    Chat { text: String, to: Option<String> },  // to=None → broadcast
    Ping,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WsServerMsg {
    Welcome { peer_id: String, meeting_id: String },
    SdpOffer { from: String, sdp: String },
    SdpAnswer { from: String, sdp: String },
    IceCandidate { from: String, candidate: String, sdp_mid: String, sdp_m_line_index: u32 },
    ParticipantJoined { peer_id: String, display_name: String, role: String },
    ParticipantLeft { peer_id: String },
    ParticipantList { participants: Vec<ParticipantInfo> },
    Muted { peer_id: String },
    Unmuted { peer_id: String },
    Kicked { peer_id: String },
    Promoted { peer_id: String },
    Chat { from: String, from_name: String, text: String, private: bool },
    Error { message: String },
    Pong,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ParticipantInfo {
    pub peer_id: String,
    pub display_name: String,
    pub role: String,
    pub is_muted: bool,
}
