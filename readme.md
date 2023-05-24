# Grace Bobber's Website!

## Video Procedure

1. Get video
2. Make sure subject is centered using keyframes if need be
3. Convert to webm using vp9 encoder and a small mp4
4. Further compress webm using ffmpeg. I used `ffmpeg -i splash-video.webm -c:v libvpx-vp9 -b:v 500k -minrate 300k -maxrate 1000k -crf 10 -c:a libvorbis output2.webm`

## Images Procedure

- New images go into /images
- run `gulp imagemin` to put images in /dist/images
- run `gulp thumbnail` to generate thumbnails for lazy loading

### If you want lazy loading

- set the `data-src` attribute to point to /dist/images and `src` to point to the thumbnail
- add lazyload and blur-up

## PDF Resume procedure

Make sure to change the `Title` in the metadata to `Grace Bobber Resume`. I used `exiftool` for this one time.

## Getting gulp working again

1. `rm -r node_modules`
2. `npm install`
3. `npm install imagemin imagemin-pngquant imagemin-zopfli imagemin-mozjpeg`
4. Profit!
