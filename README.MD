# Rust MAVLink VTOL Companion Computer Script

## Overview
This repository contains a Rust-based MAVLink script for controlling VTOL drones using PX4 SITL with Gazebo simulation.  
It is designed to later run on a real companion computer onboard a drone.


## Development Environment (Nix)

This project uses a Nix flake to set up a reproducible Rust development environment

**To get started:**

1. Install [Nix](https://nixos.org/download.html) (make sure Flakes are enabled)

2. Allow project environment:

```bash
cd rust_vtol
direnv allow
```
You only need to run `direnv` allow once.
After that, any time you enter the project folder, the environment will automatically activate

## Ardupilot Setup


https://github.com/ArduPilot/SITL_Models#
https://github.com/ArduPilot/ardupilot_gazebo

## Follow ardu_pilot README

```
export GZ_SIM_RESOURCE_PATH=$HOME/dev/ardupilot_gazebo/models:$HOME/dev/ardupilot_gazebo/worlds:\
$HOME/dev/SITL_Models/Gazebo/models:$HOME/dev/SITL_Models/Gazebo/worlds:${GZ_SIM_RESOURCE_PATH}
export GZ_SIM_SYSTEM_PLUGIN_PATH=$HOME/dev/ardupilot_gazebo/build:${GZ_SIM_SYSTEM_PLUGIN_PATH}
gz sim -v4 -r $HOME/dev/SITL_Models/Gazebo/worlds/skywalker_x8_quad_runway.sdf
```
QGC Connect to UDP on port 14550

Run ArduPilot SITL 14550 is QGC 14551 is guided script
```
cd $HOME/dev/ardupilot
./Tools/autotest/sim_vehicle.py \
    -v ArduPlane \
    --model JSON \
    --add-param-file=$HOME/dev/SITL_Models/Gazebo/config/skywalker_x8_quad.param \
    --console --map \
    --out=udp:127.0.0.1:14550 \
    --out=udp:127.0.0.1:14551
```
## PX4 Setup

### Prerequisites
- [PX4-Autopilot](https://docs.px4.io/main/en/dev_setup/building_px4.html) downloaded and built
- [QGroundControl](https://docs.qgroundcontrol.com/master/en/getting_started/download_and_install.html) installed

> **Note:**  
> PX4, Gazebo and QGroundControl are installed outside of the project environment. As they are used only for SITL

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