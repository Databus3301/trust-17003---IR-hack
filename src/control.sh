#!/bin/bash

SERIAL_PORT="/dev/ttyACM0"

# Configure the serial port
# sudo chmod a+rw $SERIAL_PORT
echo $0
echo -n $1 | minicom -o -D $SERIAL_PORT -b 9600
