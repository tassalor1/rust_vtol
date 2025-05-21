#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Phase {
    Disconnected, // no heartbeat yet
    Connected,    // got heartbeat
    Armed,        // armed flag seen
    Guided,       // in offboard mode
}
