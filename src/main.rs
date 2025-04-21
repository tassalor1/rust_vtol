use std::{error::Error, thread, time::Duration};
use mavlink::{connect, MavHeader};
use mavlink::common::{
    MavMessage,
    COMMAND_LONG_DATA, SET_POSITION_TARGET_LOCAL_NED_DATA,
    MavCmd, MavFrame, PositionTargetTypemask,
};

fn main() -> Result<(), Box<dyn Error>> {
    // Connect to SITL (listen on 14551, SITL on 14550)
    let conn = connect::<MavMessage>("udpin:0.0.0.0:14551")?;
    let header = MavHeader {
        system_id:    255,  // companion‑computer ID
        component_id: 1,
        sequence:     0,
    };

    // switch to GUIDED mode (4)
    conn.send(&header, &MavMessage::COMMAND_LONG(COMMAND_LONG_DATA {
        target_system:    1,
        target_component: 1,
        command:          MavCmd::MAV_CMD_DO_SET_MODE,
        confirmation:     0,
        param1:           4.0, // GUIDED
        param2:           0.0, param3: 0.0, param4: 0.0,
        param5:           0.0, param6: 0.0, param7: 0.0,
    }))?;

    // arm the vehicle
    conn.send(&header, &MavMessage::COMMAND_LONG(COMMAND_LONG_DATA {
        target_system:    1,
        target_component: 1,
        command:          MavCmd::MAV_CMD_COMPONENT_ARM_DISARM,
        confirmation:     0,
        param1:           1.0, // arm
        param2:           0.0, param3: 0.0, param4: 0.0,
        param5:           0.0, param6: 0.0, param7: 0.0,
    }))?;

    // give it a moment to arm
    thread::sleep(Duration::from_secs(2));

    // send a LOCAL_NED setpoint to climb 5 m
    let mask = PositionTargetTypemask::from_bits_truncate(0b0000_0111_1000);
    let pos = SET_POSITION_TARGET_LOCAL_NED_DATA {
        time_boot_ms:     0,
        target_system:    1,
        target_component: 1,
        coordinate_frame: MavFrame::MAV_FRAME_LOCAL_NED,
        type_mask:        mask,
        x:  0.0, y:  0.0, z: -5.0,
        vx: 0.0, vy: 0.0, vz: 0.0,
        afx:0.0, afy:0.0, afz:0.0,
        yaw:     0.0, yaw_rate: 0.0,
    };
    conn.send(&header, &MavMessage::SET_POSITION_TARGET_LOCAL_NED(pos))?;

    Ok(())
}
