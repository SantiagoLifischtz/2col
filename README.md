# Note: Don't use this yet! WIP

As of right now, some critical things are missing from the format.

# 2col - Video format to play Bad Apple on anything

2col encodes frame data by only saving the pixels that differ from the previous frame.

Of course, only two colors are supported. The original purpose of this format is being a compressed version of the classic [Bad Apple!!](https://www.youtube.com/watch?v=FtutLA63Cp8) music video.

### Supported input formats

Anything supported by FFmpeg! As long as it can be read as raw pixel data at 24 bits per pixel, it can be converted.

### Where do you even use this?

The whole joke of Bad Apple is to play it on anything that can display two colors. 2col is just a tool to make those meme video players slightly simpler, or at least make the step of reading the original video data easier.

### Format explanation

Frames are encoded as runs of unsigned little-endian 32-bit integers. They are seen as an array, where `y*width+x` is the index (left-right, top-down).

#### Change runs

A change run represents a set of consecutive pixels that all have changed their color since the last frame. A single change run occupies 8 bytes, the first 4 being the starting index, and the other being the amount of pixels until the next 0.

For example, a run of 12 changes starting at index 628 would be encoded as the hex values:

`74020000 0C000000`

#### Frame format

The first 4 bytes of a frame indicate the amount of change runs in that frame. Multiplying this number by 8 yields the amount of bytes that frame occupies. Right after that, the change runs of that frame are found consecutively.

Empty frames are encoded as 4 bytes set to 0, representing zero change runs are found in that frame.

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