// src/flight_mode.rs

use mavlink::common::MavModeFlag;

#[derive(Clone, Copy)]
pub enum FlightMode {
    Manual,
    Circle,
    Stabilize,
    Training,
    Acro,
    FBWA,      // Fly-By-Wire A
    FBWB,      // Fly-By-Wire B
    Cruise,
    Autotune,
    Auto,      // Mission
    RTL,
    Loiter,
    Takeoff,
    AvoidADSB,
    Guided,
}

pub struct FlightModeParams {
    /// bits for base_mode: set CUSTOM_MODE_ENABLED, plus ARM flag if you want auto-arm
    pub base_mode: u8,
    /// custom_mode = the ArduPlane FLTMODE1 value (0–15)
    pub custom_mode: u8,
}

/// ArduPlane FLTMODE1 mapping from MAVLink spec:
/// 0=MANUAL,1=CIRCLE,2=STABILIZE,3=TRAINING,4=ACRO,5=FBWA,6=FBWB,7=CRUISE,
/// 8=AUTOTUNE,10=AUTO,11=RTL,12=LOITER,13=TAKEOFF,14=AVOID_ADSB,15=GUIDED :contentReference[oaicite:0]{index=0}
pub fn flight_mode_params(mode: FlightMode) -> FlightModeParams {
    // always enable custom_mode
    let mut bm = MavModeFlag::MAV_MODE_FLAG_CUSTOM_MODE_ENABLED.bits();
    // if you want the FC disarmed initially, omit ARMDED flag; add it here if desired:
    // bm |= MavModeFlag::MAV_MODE_FLAG_SAFETY_ARMED.bits();

    let cm = match mode {
        FlightMode::Manual     => 0,
        FlightMode::Circle     => 1,
        FlightMode::Stabilize  => 2,
        FlightMode::Training   => 3,
        FlightMode::Acro       => 4,
        FlightMode::FBWA       => 5,
        FlightMode::FBWB       => 6,
        FlightMode::Cruise     => 7,
        FlightMode::Autotune   => 8,
        FlightMode::Auto       => 10,
        FlightMode::RTL        => 11,
        FlightMode::Loiter     => 12,
        FlightMode::Takeoff    => 13,
        FlightMode::AvoidADSB  => 14,
        FlightMode::Guided     => 15,
    };

    FlightModeParams { base_mode: bm, custom_mode: cm }
}
