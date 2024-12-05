export PATH=$PATH:/home/$USER/.cargo/bin

cd /home/harry/src/AOPW
#flash the code to the board
#%/home/harry/.cargo/bin/probe-rs flash wifi_tcp_server

#!/bin/bash
trap 'exit' INT

# Run the code on the board
probe-rs reset --chip rp2040 --protocol swd

sleep 2

probe-rs run --chip RP2040 --allow-erase-all wifi_tcp_server 
# probe-rs run --chip RP2040 target/thumbv6m-none-eabi/debug/wifi_tcp_server -- moved from the build folder to the deployment folder.
# ./scripts/run.sh -This is to run
# chmod +x myscript.sh -This is to make executable
