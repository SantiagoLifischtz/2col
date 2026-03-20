use std::process::{Command, Stdio, Output};
use std::{env, io};
use std::fs::File;
use std::io::Write;

type Pixel = Vec<u8>;
type Frame = Vec<Pixel>;
type PackedFrame = Vec<u64>;

fn ffmpeg_command(input: &String) -> Output {
    let output = Command::new("ffmpeg")
    .args(["-i",input,"-f","rawvideo","-vf","scale=640:360","-frames:v","500","-fpsmax","10","-pix_fmt","rgb24","-"])
    .stdout(Stdio::piped())
    .output()
    .expect("ffmpeg error occurred");

    output
}

fn group_vec<T: Clone>(v: &Vec<T>, n: usize) -> Vec<Vec<T>> {
    v.chunks(n).map(|chunk| chunk.to_vec()).collect()
}

fn white_threshold(v: &Pixel) -> bool {
    let luma = 0.299*v[0] as f32 + 0.587*v[1] as f32+ 0.114*v[2] as f32;
    luma > 128.0
}

fn pack_frame_bits(frame: &Frame) -> PackedFrame {
    const CHUNK_SIZE: usize = 64;
    let mut packed: PackedFrame = Vec::new();

    for i in 0..frame.len() {
        if i%CHUNK_SIZE == 0 {
            packed.push(0);
        }

        let is_white = white_threshold(&frame[i]);
        if is_white {
            packed[i/CHUNK_SIZE] |= 1 << (i%CHUNK_SIZE);
        }
    }

    packed
}

fn xor_packed_frames(f1: &PackedFrame, f2: &PackedFrame) -> PackedFrame {
    let mut out = vec![0; f1.len()];
    for i in 0..out.len() {
        out[i] = f1[i] ^ f2[i];
    }

    out
}

fn get_frame_string(diff: PackedFrame) -> String {
    let mut out = String::new();
    for i in 0..diff.len() {
        for j in 0..64 {
            let bit: u8 = ((diff[i] >> j) & 1) as u8;
            if bit == 1 {
                let index = (i*64+j).to_string();
                out.push_str(index.as_str());
                out.push(' ');
            }
        }
    }
    out.push('\n');

    out
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Input file required");
    }
    let input_filename = &args[1];

    println!("Running ffmpeg...");
    io::stdout().flush().unwrap();
    let output = ffmpeg_command(input_filename);
    println!(" Done");

    if output.stdout.len() == 0 {
        println!("this should not print");
    }

    print!("Grouping RGB into pixels...");
    io::stdout().flush().unwrap();
    let video_color_data = output.stdout;
    let video_pixel_data: Vec<Pixel> = group_vec(&video_color_data, 3);
    println!(" Done");
    
    print!("Grouping video into frames...");
    io::stdout().flush().unwrap();
    let frame_size = 640*360;
    let frame_data: Vec<Frame> = group_vec(&video_pixel_data, frame_size);
    println!(" Done");

    print!("Calculating frame deltas...");
    io::stdout().flush().unwrap();
    let mut diff: Vec<PackedFrame> = Vec::new();
    let mut prev_frame = pack_frame_bits(&frame_data[0]);
    
    for i in 1..frame_data.len() {
        let frame = pack_frame_bits(&frame_data[i]);
        diff.push(xor_packed_frames(&prev_frame, &frame));
        prev_frame = frame;
    }
    println!(" Done");

    print!("Writing output file...");
    io::stdout().flush().unwrap();
    let mut file = File::create("output/output.2col").unwrap();

    for frame_delta in diff {
        let line = get_frame_string(frame_delta);
        file.write_all(line.as_bytes()).unwrap();
    }
    println!(" Done");
}
