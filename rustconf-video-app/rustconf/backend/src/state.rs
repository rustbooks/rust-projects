use std::collections::HashMap;
use std::sync::Arc;

use dashmap::DashMap;
use sqlx::SqlitePool;
use tokio::sync::broadcast;
use uuid::Uuid;

/// One connected peer inside a meeting (in-memory only)
#[derive(Clone, Debug)]
pub struct Peer {
    pub id: Uuid,
    pub display_name: String,
    pub role: String,            // "host" | "participant"
    pub is_muted: bool,
    pub is_video_off: bool,
    /// Channel to push server-side events (mute, kick, chat) to this peer's WS
    pub tx: broadcast::Sender<String>,
}

/// In-memory state for a single meeting room
pub struct Room {
    pub meeting_id: String,
    pub host_id: Uuid,
    /// peer_id → Peer
    pub peers: DashMap<Uuid, Peer>,
    /// Broadcast channel for room-wide events (chat, participant list)
    pub room_tx: broadcast::Sender<String>,
}

impl Room {
    pub fn new(meeting_id: String, host_id: Uuid) -> Self {
        let (room_tx, _) = broadcast::channel(512);
        Self {
            meeting_id,
            host_id,
            peers: DashMap::new(),
            room_tx,
        }
    }
}

pub struct AppState {
    pub db: SqlitePool,
    /// meeting_id → Room
    pub rooms: DashMap<String, Arc<Room>>,
    pub jwt_secret: String,
}

impl AppState {
    pub fn new(db: SqlitePool) -> Self {
        let jwt_secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "change-me-in-production-32-chars!!".to_string());
        Self {
            db,
            rooms: DashMap::new(),
            jwt_secret,
        }
    }

    /// Get or create in-memory room for a meeting
    pub fn get_or_create_room(&self, meeting_id: &str, host_id: Uuid) -> Arc<Room> {
        self.rooms
            .entry(meeting_id.to_string())
            .or_insert_with(|| Arc::new(Room::new(meeting_id.to_string(), host_id)))
            .clone()
    }
}
