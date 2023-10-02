#!/bin/bash

sudo modprobe -r v4l2loopback
sudo modprobe v4l2loopback exclusive_caps=1

FONT='/usr/share/fonts/noto/NotoSansMono-Regular.ttf'
INPUT='/dev/video0'
OUTPUT='/dev/video1'

TEMP="/tmp/temp-$$.png"

(
    trap 'kill 0' SIGINT;
    cargo run --release -- -f "$FONT" -F 12 -W 1280 -H 720 "$INPUT" "$TEMP" &
    sleep 1;
    ffmpeg -re -stream_loop -1 -i "$TEMP" -f v4l2 -pix_fmt yuv420p -s 1280x720 "$OUTPUT"
)
