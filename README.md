# Note: Don't use this yet! WIP

As of right now, the format is very big, way bigger than it should be.

# 2col - Video format to play Bad Apple on anything

2col encodes frame data by only saving the pixels that differ from the previous frame.

Of course, only two colors are supported. The original purpose of this format is being a compressed version of the classic [Bad Apple!!](https://www.youtube.com/watch?v=FtutLA63Cp8) music video.

### Supported input formats

Anything supported by FFmpeg! As long as it can be read as raw pixel data at 24 bits per pixel, it can be converted.

### Where do you even use this?

The whole joke of Bad Apple is to play it on anything that can display two colors. 2col is just a tool to make those meme video players slightly simpler, or at least make the step of reading the original video data easier.

### Format explanation

Frames are encoded as runs of little-endian 32-bit integers. The first four bytes of every frame indicate the amount of pixels that were changed. Each following group of four bytes is an index of a pixel that was changed since the last frame.

### How to use

First, [install FFmpeg](https://www.ffmpeg.org/) if you haven't already, as the converter depends on it.

This project was made in Rust, you can build it anywhere. The example here is for Linux.

You can either direlctly run the project like this
```bash
cargo run -- path/to/input_file
```
Or build it and run the binary like this
```bash
cargo build --release
target/release/twocol path/to/input_file
```
You can then use the executable anywhere after that

### Roadmap

Short term: 
- Compress the format
- Add support for any output resolution and framerate
- Add the two colors, resolution and framerate into the file.

Long term:
- KEEP COMPRESSING
- Make a tool to convert back to other video formats
- Make a simple 2col video player