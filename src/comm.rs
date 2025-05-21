use std::sync::Arc;
use tokio::sync::Mutex;
use mavlink::{
    MavHeader,
    common::{HEARTBEAT_DATA, MavMessage, MavType, MavAutopilot, MavModeFlag, MavState},
};
use crate::{phase::Phase, flight_mode::{flight_mode_params, FlightMode}};


type Conn = Box<dyn mavlink::MavConnection<MavMessage> + Send + Sync>;

/// RX task: read heartbeats and update `phase`
pub async fn heartbeat_rx(
    conn: Arc<Mutex<Conn>>,
    phase: Arc<Mutex<Phase>>,
) {
    let off_main = flight_mode_params(FlightMode::Offboard).main_mode;
    loop {
        // destructure header+message
        let pair = conn.lock().await.recv();
        let (_hdr, msg) = match pair {
            Ok(p) => p,
            Err(_) => continue,
        };

        if let MavMessage::HEARTBEAT(hb) = msg {
            let mut ph = phase.lock().await;
            match *ph {
                Phase::Disconnected => {
                    *ph = Phase::Connected;
                    println!("→ Connected");
                }
                Phase::Connected
                    if hb.base_mode.contains(MavModeFlag::MAV_MODE_FLAG_SAFETY_ARMED) =>
                {
                    *ph = Phase::Armed;
                    println!("→ Armed");
                }
                Phase::Armed
                    if ((hb.custom_mode >> 16) & 0xFF) as u8 == off_main =>
                {
                    *ph = Phase::Guided;
                    println!("→ Guided (Offboard)");
                }
                _ => {}
            }
        }
    }
}

/// TX task: send GCS heartbeat at 10 Hz
pub async fn heartbeat_tx(conn: Arc<Mutex<Conn>>) {
    let mut seq = 0u8;
    let mut tick = tokio::time::interval(std::time::Duration::from_millis(100));

    loop {
        tick.tick().await;
        let hb = MavMessage::HEARTBEAT(HEARTBEAT_DATA {
            custom_mode:    0,
            mavtype:        MavType::MAV_TYPE_GCS,
            autopilot:      MavAutopilot::MAV_AUTOPILOT_INVALID,
            base_mode:      MavModeFlag::empty(),
            system_status:  MavState::MAV_STATE_ACTIVE,
            mavlink_version: 3,
        });
        let hdr = MavHeader { system_id:1, component_id:191, sequence:seq };
        let _ = conn.lock().await.send(&hdr, &hb);
        seq = seq.wrapping_add(1);
    }
}
