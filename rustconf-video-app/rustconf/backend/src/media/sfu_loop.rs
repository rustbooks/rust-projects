/// str0m SFU Drive Loop — reference implementation
///
/// str0m is I/O-free: you own the UDP socket. This file shows the complete
/// event loop that you spawn per-peer when they WHIP in.
///
/// Paste this into media/sfu.rs and call `spawn_peer_loop()` from whip.rs
/// once you have the SDP negotiation working.

use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::Arc,
    time::{Duration, Instant},
};

use dashmap::DashMap;
use str0m::{
    change::{DtlsCert, SdpOffer},
    media::{KeyframeRequestKind, MediaData, Mid},
    net::{Protocol, Receive},
    Event, IceConnectionState, Input, Output, Rtc,
};
use tokio::{net::UdpSocket, sync::mpsc};
use tracing::{debug, info, warn};
use uuid::Uuid;

/// Shared SFU state: peer_id → RTP sender channel
pub type PeerMap = Arc<DashMap<Uuid, mpsc::Sender<MediaData>>>;

pub struct PeerSession {
    pub peer_id: Uuid,
    pub meeting_id: String,
    pub rtc: Rtc,
    pub socket: Arc<UdpSocket>,
    pub remote_addr: SocketAddr,
    pub peers: PeerMap, // shared reference to forward media to other peers
}

/// Spawn the UDP/RTP drive loop for a single peer.
/// Call from whip_offer() after SDP negotiation.
pub async fn spawn_peer_loop(session: PeerSession) {
    tokio::spawn(async move {
        run_peer(session).await;
    });
}

async fn run_peer(mut session: PeerSession) {
    let mut buf = vec![0u8; 2048];

    loop {
        // Calculate when str0m wants its next timeout
        let timeout = match session.rtc.poll_output().unwrap() {
            Output::Timeout(t) => t,
            Output::Transmit(send) => {
                // Forward outbound packet back to the peer
                if let Err(e) = session.socket.send_to(&send.contents, session.remote_addr).await {
                    warn!("UDP send error: {e}");
                }
                continue;
            }
            Output::Event(event) => {
                handle_event(&mut session, event).await;
                continue;
            }
        };

        let now = Instant::now();
        let deadline = timeout.saturating_duration_since(now);

        tokio::select! {
            // Inbound UDP packet
            result = session.socket.recv_from(&mut buf) => {
                match result {
                    Ok((n, addr)) => {
                        session.remote_addr = addr;
                        let input = Input::Receive(
                            Instant::now(),
                            Receive {
                                proto: Protocol::Udp,
                                source: addr,
                                destination: session.socket.local_addr().unwrap(),
                                contents: buf[..n].to_vec().try_into().unwrap(),
                            },
                        );
                        if let Err(e) = session.rtc.handle_input(input) {
                            warn!("str0m input error: {e}");
                            break;
                        }
                    }
                    Err(e) => {
                        warn!("UDP recv error: {e}");
                        break;
                    }
                }
            }

            // str0m timeout tick
            _ = tokio::time::sleep(deadline) => {
                if let Err(e) = session.rtc.handle_input(Input::Timeout(Instant::now())) {
                    warn!("str0m timeout error: {e}");
                    break;
                }
            }
        }

        // Check if the ICE connection died
        if session.rtc.ice_connection_state() == IceConnectionState::Disconnected {
            info!("Peer {} disconnected", session.peer_id);
            break;
        }
    }

    // Cleanup: remove from shared peer map
    session.peers.remove(&session.peer_id);
}

async fn handle_event(session: &mut PeerSession, event: Event) {
    match event {
        Event::IceConnectionStateChange(state) => {
            info!("Peer {} ICE state: {:?}", session.peer_id, state);
        }

        Event::MediaData(data) => {
            // Forward RTP data to all other peers in the meeting
            for entry in session.peers.iter() {
                if *entry.key() == session.peer_id {
                    continue; // Don't echo back to sender
                }
                let _ = entry.value().try_send(data.clone());
            }
        }

        Event::KeyframeRequest { mid, kind } => {
            // PLI / FIR: request keyframe from the sender
            if kind == KeyframeRequestKind::Pli {
                debug!("PLI for mid {:?} from peer {}", mid, session.peer_id);
                // In full impl: forward PLI to the publisher peer
            }
        }

        _ => {}
    }
}
