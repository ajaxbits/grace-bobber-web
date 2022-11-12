ffmpeg -i videos/test-video.mp4 \
    -filter:v "vflip,crop=720:720:525:0,unsharp=5:5:1.0" \
    -c:v libvpx-vp9 \
    -crf 40 \
    -deadline best \
    videos/new-splash.webm

ffmpeg -i videos/test-video.mp4 \
    -filter:v "vflip,crop=720:720:525:0,unsharp=5:5:1.0" \
    -c:v libx265 \
    -crf 32 \
    -preset veryslow \
    -movflags faststart \
    -tag:v hvc1 \
    videos/new-splash.mp4

ffmpeg -i videos/new-splash.webm -ss 1 -vframes 1 images/splash-vid-first-frame.jpg
