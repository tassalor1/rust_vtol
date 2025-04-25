
PREREQUISITES
[ardupilot](https://github.com/ArduPilot/ardupilot.git) installed and dev enviroment setup
[gazebo](https://gazebosim.org/docs/garden/install_ubuntu/)

INSTALL Ardu Gazebo repos
```
https://github.com/ArduPilot/SITL_Models#
https://github.com/ArduPilot/ardupilot_gazebo
```
Follow ardu_pilot README

Run Gazebo
```
cd $HOME/SITL_Models/Gazebo/worlds
gz sim -v4 -r skywalker_x8_quad_runway.sdf
```

QGC Connect to UDP on port 14550

Run ArduPilot SITL
14550 is QGC 14551 is guided script
```
cd ~/ardupilot
/Tools/autotest/sim_vehicle.py   -v ArduPlane   --model JSON   --add-param-file=$HOME/dev/SITL_Models/Gazebo/config/skywalker_x8.param   --console --map   --out=udp:127.0.0.1:14550   --out=udp:127.0.0.1:14551   --no-rebuild  
```

Guided Script
```
cargo run
```