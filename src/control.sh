#!/bin/bash

SERIAL_PORT="/dev/ttyACM0"

# Configure the serial port
sudo chmod a+rw $SERIAL_PORT
sleep 3
sudo fuser -k $SERIAL_PORT
sleep 3
stty -F $SERIAL_PORT 9600 cs8 -cstopb -parenb
sleep 3
echo -n 'l' > $SERIAL_PORT
