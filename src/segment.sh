#!/bin/bash

rm -rf segments
mkdir segments

ffmpeg -i ./capture/output.mp4 ./segments/frame_%04d.jpg
