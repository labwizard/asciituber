#!/bin/bash

sudo modprobe -r v4l2loopback
sudo modprobe v4l2loopback exclusive_caps=1

cargo build --release

FONT='/usr/share/fonts/noto/NotoSansMono-Regular.ttf'
INPUT='/dev/video0'
OUTPUT='/dev/video2'
WIDTH=1280
HEIGHT=720

(
    trap "rm $TEMP; kill 0" SIGINT
    cargo run --release -- \
        -f "$FONT" -F 12 -W "$WIDTH" -H "$HEIGHT" \
        --alt-fg=202020 \
        "$INPUT" - \
    | ffmpeg -f image2pipe -c:v mjpeg -i - -f v4l2 -pix_fmt yuv420p "$OUTPUT"
)
