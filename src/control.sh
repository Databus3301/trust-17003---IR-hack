#!/bin/bash

SERIAL_PORT="/dev/ttyACM0"

# Configure the serial port
sudo chmod a+rw $SERIAL_PORT
sudo fuser -k $SERIAL_PORT
stty -F $SERIAL_PORT 9600 cs8 -cstopb -parenb

echo "l" > $SERIAL_PORT
