/// WHIP (WebRTC-HTTP Ingestion Protocol) endpoint
///
/// Browsers POST an SDP offer here. The server (str0m) processes it,
/// creates the peer connection, and returns an SDP answer.
///
/// For 300 participants at SFU scale:
///   • Each publisher sends 1 video+audio track UP to the SFU
///   • SFU forwards N-1 streams DOWN to each subscriber
///   • No mixing, no transcoding → minimal CPU
///
/// str0m integration notes:
///   str0m is an I/O-free WebRTC library – you feed it UDP packets and
///   it gives you decrypted RTP. You own the UDP socket loop. Below is
///   the complete scaffolding wired to Axum.
use std::sync::Arc;

use axum::{
    body::Bytes,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use tracing::info;

use crate::state::AppState;

/// POST /whip/:meeting_id/:peer_id
/// Body: SDP offer (application/sdp)
/// Returns: 201 + SDP answer
pub async fn whip_offer(
    State(_state): State<Arc<AppState>>,
    Path((meeting_id, peer_id)): Path<(String, String)>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    let sdp_offer = match std::str::from_utf8(&body) {
        Ok(s) => s.to_string(),
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid UTF-8").into_response(),
    };

    info!("WHIP offer from peer {} in meeting {}", peer_id, meeting_id);

    // ── str0m integration (production wiring) ────────────────────────────
    //
    // use str0m::{Rtc, Input, Output, Event, Change};
    // use str0m::net::Protocol;
    // use std::net::SocketAddr;
    //
    // let rtc = Rtc::builder()
    //     .set_ice_lite(true)          // SFU mode: clients connect TO us
    //     .build();
    //
    // // Parse the SDP offer
    // let offer = SdpOffer::from_sdp_string(&sdp_offer).unwrap();
    // let answer = rtc.sdp_api().accept_offer(offer).unwrap();
    //
    // // Store rtc handle keyed by peer_id
    // // Then run the UDP drive loop in a spawned task (see docs/str0m_loop.rs)
    //
    // return (
    //     StatusCode::CREATED,
    //     [("Content-Type", "application/sdp"),
    //      ("Location", &format!("/whip/{}/{}", meeting_id, peer_id))],
    //     answer.to_sdp_string(),
    // ).into_response();
    // ─────────────────────────────────────────────────────────────────────

    // Stub response until str0m is wired up end-to-end
    (
        StatusCode::NOT_IMPLEMENTED,
        "str0m WHIP endpoint – wire up RtcHandle map (see media/whip.rs comments)",
    )
        .into_response()
}

/// PATCH /whip/:meeting_id/:peer_id
/// Body: Trickle ICE candidate (application/trickle-ice-sdpfrag)
pub async fn whip_ice(
    State(_state): State<Arc<AppState>>,
    Path((meeting_id, peer_id)): Path<(String, String)>,
    body: Bytes,
) -> impl IntoResponse {
    info!("ICE trickle from {} in {}", peer_id, meeting_id);
    // In production: parse candidate, hand to rtc.add_local_candidate()
    StatusCode::NO_CONTENT
}
