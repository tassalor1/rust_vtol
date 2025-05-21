mod phase;
mod flight_mode;
mod comm;
mod setpoint;

use mavlink::{common::MavMessage, connect};
use std::{sync::Arc, future};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let uri = "udpin:0.0.0.0:14551";
    

    let raw = connect::<MavMessage>(uri).expect("mavlink connect");
    let shared_conn = Arc::new(Mutex::new(raw));
    let shared_phase = Arc::new(Mutex::new(phase::Phase::Disconnected));

    tokio::spawn(comm::heartbeat_rx(shared_conn.clone(), shared_phase.clone()));
    tokio::spawn(comm::heartbeat_tx(shared_conn.clone()));
    tokio::spawn(setpoint::setpoint_loop(shared_conn.clone(), shared_phase.clone()));

    future::pending::<()>().await;
}
