
INSTALL
```
https://github.com/ArduPilot/SITL_Models#
https://github.com/ArduPilot/ardupilot_gazebo
```
Run Gazebo
```
cd $HOME/SITL_Models/Gazebo/worlds
gz sim -v4 -r skywalker_x8_quad_runway.sdf
```
Run ArduPilot SITL
```
cd ~/ardupilot
./Tools/autotest/sim_vehicle.py -v ArduPlane --model JSON --add-param-file=$HOME/SITL_Models/Gazebo/config/skywalker_x8.param --console --map
```

QGC Connect to Port