#!/bin/bash

vid=$(v4l2-ctl --list-devices | grep -A 1 "Trust" | grep "video" | cut -d '/' -f 3)

ffmpeg -f v4l2 -i /dev/"$vid" -pix_fmt yuyv422 -f rawvideo - | ffplay -fs -f rawvideo -pixel_format yuyv422 -video_size 640x480 -
