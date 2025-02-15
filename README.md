# 2024_winter_main
2024_winter_robo_ws

git status で，masterブランチの最新版であることを確認すること

- git switch origin master
- git pull

# command
- colcon build --symlink-install
- source install/setup.bash

robot_1 コントローラー側
- ros2 joy joy_joynode

robot_1 本体側
- apt install libasound2-dev libudev-dev　pkg-config
- ros2 robot_1 robot_1

robot_2 コントローラー側
- ros2 launch robot_2_launch robot_2_launch robot_2_launch.xml みたいな

robot_2 本体側
- ros2 run robot_2 robot_2
- ros2 run robot_2_controller robot_2_controller
