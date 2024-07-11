#!/bin/bash

SERIAL_PORT="/dev/ttyACM0"

# Configure the serial port
sudo chmod a+rw $SERIAL_PORT

echo -n $1 | minicom -o -D /dev/ttyACM0 -b 9600
