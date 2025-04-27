use mavlink::{
    connect, MavHeader,
    common::{
        HEARTBEAT_DATA, SET_POSITION_TARGET_LOCAL_NED_DATA,
        MavMessage, MavType, MavAutopilot, MavState,
        MavModeFlag, MavFrame, PositionTargetTypemask as Mask,
    },
};
use std::{
    error::Error,
    sync::{Arc, Mutex},
};
use tokio::time::{sleep, Duration, interval};

mod flight_mode;
use flight_mode::{FlightMode, flight_mode_params};

#[derive(Clone, Copy, PartialEq, Debug)]
enum Phase { Disconnected, Connected, Armed, Guided }

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let port = "14540";
    let conn_str = format!("udpin:0.0.0.0:{}", port);
    let link  = Arc::new(Mutex::new(connect::<MavMessage>(&conn_str)?));
    let phase = Arc::new(Mutex::new(Phase::Disconnected));
    

    println!("✓ MAVLink listening on {conn_str}");

    /* ----------- TASK 1: pump vehicle heartbeats ----------- */
    {
        let (link, phase) = (link.clone(), phase.clone());
        tokio::spawn(async move {
            // assign modes
            let offboard = flight_mode_params(FlightMode::Offboard); 
            let manual = flight_mode_params(FlightMode::Manual); 
            let acro = flight_mode_params(FlightMode::Acro); 
            loop {
                let msg = {
                    // blocking read, but in its own task
                    let c = link.lock().unwrap();
                    match c.recv() {
                        Ok((_, m)) => m,
                        Err(_)     => continue,
                    }
                };

                if let MavMessage::HEARTBEAT(vhb) = &msg {
                    if vhb.mavtype != MavType::MAV_TYPE_GCS {
                        // px4 way of parsing mavlink - get the bit of these modes
                        let main_mode = ((vhb.custom_mode >> 16) & 0xFF) as u8;
                        let sub_mode  = ((vhb.custom_mode >> 24) & 0xFF) as u8;
                        let mut st = phase.lock().unwrap();
                        match *st {
                            Phase::Disconnected => {
                                *st = Phase::Connected;
                                println!("✓ vehicle connected");
                            }
                            Phase::Connected if
                                vhb.base_mode.contains(MavModeFlag::MAV_MODE_FLAG_SAFETY_ARMED) =>
                            {
                                *st = Phase::Armed;
                                println!("✓ vehicle armed");
                            }
                            Phase::Armed if
                                vhb.base_mode.contains(MavModeFlag::MAV_MODE_FLAG_SAFETY_ARMED) &&
                                main_mode == offboard.main_mode =>               
                            {
                                *st = Phase::Guided;
                                println!("✓ {offboard:?} mode confirmed");
                            }
                            _ => {}
                        }
                    }
                }
            }
        });
    }

    /* ------------- TASK 2: send GCS heartbeat to FC-------------- */
    {
        let link = link.clone();
        tokio::spawn(async move {
            let mut seq = 0u8;
            let mut hb_tick = interval(Duration::from_millis(100)); // 10 Hz
            loop {
                hb_tick.tick().await;
                let hb = MavMessage::HEARTBEAT(HEARTBEAT_DATA {
                    custom_mode: 0,
                    mavtype:      MavType::MAV_TYPE_GCS,
                    autopilot:    MavAutopilot::MAV_AUTOPILOT_INVALID,
                    base_mode:    MavModeFlag::empty(),
                    system_status:MavState::MAV_STATE_ACTIVE,
                    mavlink_version: 3,
                });
                link.lock().unwrap()
                    .send(&MavHeader{system_id:1, component_id:191, sequence:seq}, &hb).ok();
                seq = seq.wrapping_add(1);
            }
        });
    }

    /* ------------- TASK 3: 10 Hz set-point stream ---------- */
    let mut seq = 0u8;
    let mut t_ms: u32 = 0;
    let mut sp_tick = interval(Duration::from_millis(100));
    loop {
        sp_tick.tick().await;
        t_ms += 100;

        let sp = MavMessage::SET_POSITION_TARGET_LOCAL_NED(
            SET_POSITION_TARGET_LOCAL_NED_DATA {
                time_boot_ms: t_ms,
                target_system:    1,
                target_component: 1,
                coordinate_frame: MavFrame::MAV_FRAME_LOCAL_NED,
                /* pos-only mask */
                type_mask:  Mask::POSITION_TARGET_TYPEMASK_VX_IGNORE
                           | Mask::POSITION_TARGET_TYPEMASK_VY_IGNORE
                           | Mask::POSITION_TARGET_TYPEMASK_VZ_IGNORE
                           | Mask::POSITION_TARGET_TYPEMASK_AX_IGNORE
                           | Mask::POSITION_TARGET_TYPEMASK_AY_IGNORE
                           | Mask::POSITION_TARGET_TYPEMASK_AZ_IGNORE
                           | Mask::POSITION_TARGET_TYPEMASK_YAW_IGNORE
                           | Mask::POSITION_TARGET_TYPEMASK_YAW_RATE_IGNORE,
                x: 0.0, y: 0.0, z: -5.0,           // hover 5 m (NED)
                ..Default::default()
            }
        );

        link.lock().unwrap()
            .send(&MavHeader{system_id:1, component_id:191, sequence:seq}, &sp).ok();
        seq = seq.wrapping_add(1);

        match *phase.lock().unwrap() {
            Phase::Disconnected => println!("waiting for heartbeat…"),
            Phase::Connected   => println!("connected — arm in QGC"),
            Phase::Armed       => println!("armed — switch to OFFBOARD"),
            Phase::Guided      => {}
        }

        sleep(Duration::from_millis(20)).await;
    }
}

