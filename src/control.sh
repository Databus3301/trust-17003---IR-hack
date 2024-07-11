#!/bin/bash

SERIAL_PORT="/dev/ttyACM0"

# Configure the serial port
stty -F $SERIAL_PORT 9600 cs8 -cstopb -parenb

echo "l" > $SERIAL_PORT
