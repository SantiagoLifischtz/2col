# Note: Don't use this yet! WIP

# 2col - Video format to play Bad Apple on anything

2col encodes frame data by only saving the pixels that differ from the previous frame.

Of course, only two colors are supported. The original purpose of this format is being a compressed version of the classic [Bad Apple!!](https://www.youtube.com/watch?v=FtutLA63Cp8) music video.

### Supported input formats

Anything supported by FFmpeg! As long as it can be read as raw pixel data at 24 bits per pixel, it can be converted.

### Where do you even use this?

The whole joke of Bad Apple is to play it on anything that can display two colors. 2col is just a tool to make those meme video players slightly simpler, or at least make the step of reading the original video data easier.

### Format explanation

Each line in the output file is a set of indices, calculated as `x * width + y`. They correspond to the pixels that have swapped color since the last frame, the rest staying the same.

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
- Add support for any output resolution and framerate
- Add the two colors, resolution and framerate into the file.

Long term:
- Make a tool to convert back to other video formats
- Make a simple 2col video player