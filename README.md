# rffmpeg

[![crates.io](https://img.shields.io/crates/v/rffmpeg)](https://crates.io/crates/rffmpeg)
[![docs.rs](https://docs.rs/rffmpeg/badge.svg)](https://docs.rs/rffmpeg)
[![build](https://github.com/nrbnlulu/rffmpeg/workflows/build/badge.svg)](https://github.com/nrbnlulu/rffmpeg/actions)

currently supports ffmpeg 6.1 - 7.1

Build instructions can be found on the [wiki](https://github.com/nrbnlulu/rffmpeg/wiki/Notes-on-building).

Documentation:

- [docs.rs](https://docs.rs/rffmpeg/);
- [FFmpeg user manual](https://ffmpeg.org/ffmpeg-all.html);
- [FFmpeg Doxygen](https://ffmpeg.org/doxygen/trunk/).

### Installation

As a general note take a look on the build.yml
for a better reference on how to build the library.

#### Windows

1. download the version you want from [ffmpeg-builds](https://github.com/BtbN/FFmpeg-Builds/releases) (only shared builds are supported)
2. extract and set `FFMPEG_DIR` `FFMPEG_INCLUDE_DIR` and `FFMPEG_LIB_DIR` environment variables to the extracted directory



#### Linux

1. download the version you want from [ffmpeg-builds](https://github.com/BtbN/FFmpeg-Builds/releases) (only shared builds are supported)
2. add these paths (change to fit your needs) to env
```bash
export FFMPEG_DIR=/home/dev/Downloads/ffmpeg
export FFMPEG_PATH=/home/dev/Downloads/ffmpeg/bin
export FFMPEG_INCLUDE_DIR=/home/dev/Downloads/ffmpeg/include
export FFMPEG_LIB_DIR=/home/dev/Downloads/ffmpeg/lib
export LD_LIBRARY_PATH=${FFMPEG_LIB_DIR}:${LD_LIBRARY_PATH}
```


#### MacOS

1. run `brew install ffmpeg pkg-config`
2. good to go
