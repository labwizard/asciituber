# asciituber

Transform your camera into ASCII output.

The program `asciituber` reads from the webcam and produces PNGs of ASCII
output; `start.sh` casts it to a virtual webcam (unfortunately, this requires
the kernel module `v4l2loopback` to be loaded and `ffmpeg` to be installed).

To do:
- Add color customization;
- introduce cross-platform support (unlikely)
