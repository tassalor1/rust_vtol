use std::sync::Arc;
use tokio::{sync::Mutex, time::{interval, sleep, Duration}};
use mavlink::{MavHeader, common::{SET_POSITION_TARGET_LOCAL_NED_DATA, MavMessage, MavFrame, PositionTargetTypemask as Mask}};
use crate::phase::Phase;

// same boxed trait
type Conn = Box<dyn mavlink::MavConnection<MavMessage> + Send + Sync>;

/// every 100 ms, if Guided, send a hover setpoint
pub async fn setpoint_loop(
    conn: Arc<Mutex<Conn>>,
    phase: Arc<Mutex<Phase>>,
) {
    let mut seq = 0u8;
    let mut tick = interval(Duration::from_millis(100));

    loop {
        tick.tick().await;
        if *phase.lock().await != Phase::Guided {
            sleep(Duration::from_millis(20)).await;
            continue;
        }

        let t_ms = seq as u32 * 100;
        let data = SET_POSITION_TARGET_LOCAL_NED_DATA {
            time_boot_ms:     t_ms,
            target_system:    1,
            target_component: 1,
            coordinate_frame: MavFrame::MAV_FRAME_LOCAL_NED,
            type_mask: Mask::POSITION_TARGET_TYPEMASK_VX_IGNORE
                     | Mask::POSITION_TARGET_TYPEMASK_VY_IGNORE
                     | Mask::POSITION_TARGET_TYPEMASK_VZ_IGNORE
                     | Mask::POSITION_TARGET_TYPEMASK_AX_IGNORE
                     | Mask::POSITION_TARGET_TYPEMASK_AY_IGNORE
                     | Mask::POSITION_TARGET_TYPEMASK_AZ_IGNORE
                     | Mask::POSITION_TARGET_TYPEMASK_YAW_IGNORE
                     | Mask::POSITION_TARGET_TYPEMASK_YAW_RATE_IGNORE,
            x:0.0, y:0.0, z:-5.0,
            ..Default::default()
        };
        let hdr = MavHeader { system_id:1, component_id:191, sequence:seq };
        let _ = conn.lock().await.send(&hdr, &MavMessage::SET_POSITION_TARGET_LOCAL_NED(data));
        seq = seq.wrapping_add(1);
    }
}
