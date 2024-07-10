#!/bin/bash

ffmpeg -f v4l2 -i /dev/video2 -f rawvideo -pix_fmt yuyv422 - | ffplay -f rawvideo -infbuf -framedrop -fs -pixel_format yuyv422 -video_size 640x480 - &
sleep 4
maim -u -f png -q -i `xdotool getactivewindow` > "$PWD"/sc.png &
