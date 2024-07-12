#!/bin/bash

sudo chmod a+rw /dev/ttyACM0

cd ./server
./target/debug/server
cd ..
