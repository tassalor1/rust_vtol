#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FlightMode {
    Manual,
    Offboard,
    Hold,
    Mission,
    RTL,
    Acro,
}

#[derive(Clone, Copy, Debug)]
pub struct FlightModeParams {
    pub base_mode: u8,
    pub main_mode: u8,
}

pub fn flight_mode_params(mode: FlightMode) -> FlightModeParams {
    match mode {
        FlightMode::Manual => FlightModeParams { base_mode: 217, main_mode: 1 },
        FlightMode::Offboard => FlightModeParams { base_mode: 209, main_mode: 6 },
        FlightMode::Hold => FlightModeParams { base_mode: 217, main_mode: 4 },
        FlightMode::Mission => FlightModeParams { base_mode: 157, main_mode: 4 },
        FlightMode::RTL => FlightModeParams { base_mode: 157, main_mode: 4 },
        FlightMode::Acro => FlightModeParams { base_mode: 209, main_mode: 5},
    }
}
