# Rust MAVLink VTOL Companion Computer Script

## Overview
This repository contains a Rust-based MAVLink script for controlling VTOL drones using PX4 SITL with Gazebo simulation.  
It is designed to later run on a real companion computer onboard a drone.

---

## PX4 Setup

### Prerequisites
- [PX4-Autopilot](https://docs.px4.io/main/en/dev_setup/building_px4.html) downloaded and built
- [QGroundControl](https://docs.qgroundcontrol.com/master/en/getting_started/download_and_install.html) installed

---

### Running PX4 SITL with Gazebo

Open a terminal and run:

```bash
cd PX4-Autopilot
make px4_sitl gz_standard_vtol
```
This will launch the PX4 SITL simulation with a standard VTOL model.

### Running QGroundControl
Start QGroundControl.
It should automatically connect to the simulator via UDP (port 14540 for PX4).

### Running the PX4 Offboard Rust Script
In a separate terminal, run:

```bash
cd rust_vtol
cargo run
```
This will start the companion computer script and begin MAVLink communication with PX4.