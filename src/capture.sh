#!/bin/bash

rm -rf capture
mkdir capture

vid=$(v4l2-ctl --list-devices | grep -A 1 "Trust" | grep "video" | cut -d '/' -f 3)

ffmpeg -f v4l2 -i /dev/"$vid" -frames:v 5 -c:v libx264 -preset fast -pix_fmt yuv420p ./capture/output.mp4
