use std::process::{ChildStdout, Command, Stdio};
use std::{env, vec};
use std::fs::File;
use std::fs;
use std::io::{Read, Write};

type Pixel = Vec<u8>;
type Frame = Vec<Pixel>;
type FrameBitmap = Vec<u64>;

#[derive(Clone, Copy)]
struct ChangeRun {
    start: u32,
    length: u32,
}

impl ChangeRun {
    pub fn new() -> Self {
        Self {
            start: 0,
            length: 0
        }
    }
    
    pub fn to_le_bytes(&self) -> [u8; 8] {
        let mut bytes = [0u8; 8];
        bytes[..4].copy_from_slice(&self.start.to_le_bytes());
        bytes[4..].copy_from_slice(&self.length.to_le_bytes());
        bytes
    }
}

fn ffmpeg_stream(input: &String) -> ChildStdout {
    let mut child = Command::new("ffmpeg")
    .args(["-i",input,"-f","rawvideo","-vf","scale=640:360","-fpsmax","10","-pix_fmt","rgb24","-"])
    .stdout(Stdio::piped())
    .spawn()
    .unwrap();

    child.stdout.take().unwrap()
}

fn group_vec<T: Clone>(v: &Vec<T>, n: usize) -> Vec<Vec<T>> {
    v.chunks(n).map(|chunk| chunk.to_vec()).collect()
}

fn white_threshold(v: &Pixel) -> bool {
    let luma = 0.299*v[0] as f32 + 0.587*v[1] as f32+ 0.114*v[2] as f32;
    luma > 128.0
}

fn pack_pixels(frame_pixels: &Frame) -> FrameBitmap {
    const CHUNK_SIZE: usize = 64;
    let cluster_count: usize = (frame_pixels.len() + CHUNK_SIZE - 1) / CHUNK_SIZE;

    let mut packed: FrameBitmap = vec![0u64; cluster_count];

    for i in 0..frame_pixels.len() {
        if white_threshold(&frame_pixels[i]) {
            packed[i/CHUNK_SIZE] |= 1 << (i%CHUNK_SIZE);
        }
    }

    packed
}

fn xor_bitmaps(f1: &FrameBitmap, f2: &FrameBitmap) -> FrameBitmap {
    let mut out = vec![0; f1.len()];
    for i in 0..out.len() {
        out[i] = f1[i] ^ f2[i];
    }

    out
}

fn get_change_runs(diff: &FrameBitmap) -> Vec<ChangeRun> {
    let mut out: Vec<ChangeRun> = Vec::new();

    let mut current_run: ChangeRun = ChangeRun::new();

    for i in 0..diff.len() {
        for j in 0..64 {
            let bit: u8 = ((diff[i] >> j) & 1) as u8;
            if bit == 1 {
                // Found a 1
                if current_run.length == 0 {
                    // Start a new run if we weren't on one
                    let index: u32 = (i as u32)*64+j;
                    current_run.start = index;
                }
                // Otherwise continue the run
                current_run.length += 1;
            }
            else {
                // Found a 0
                if current_run.length > 0 {
                    // End and save the run if we were on one 
                    out.push(current_run);
                    current_run = ChangeRun::new();
                }
            }
        }
    }

    out
}

fn get_frame_change_bytes(change_runs: &Vec<ChangeRun>) -> Vec<u8> {
    let mut out = Vec::new();

    out.extend_from_slice(&(change_runs.len() as u32).to_le_bytes());

    for &cr in change_runs {
        out.extend_from_slice(&cr.to_le_bytes());
    }

    out
}

fn get_frame_bitmap(rgb24_frame: &Vec<u8>) -> FrameBitmap {
    let pixels = group_vec(rgb24_frame, 3);
    
    pack_pixels(&pixels)
}

fn run_pipeline(stream: &mut ChildStdout, frame_byte_size: usize, output_file: &mut File) {
    let mut buffer = vec![0u8; frame_byte_size];

    // Taking advantage of the starting all 0 buffer to generate an all 0 bitmap
    let mut prev_frame = get_frame_bitmap(&buffer);

    loop {
        match stream.read_exact(&mut buffer) {
            Ok(_) => {

                let bitmap = get_frame_bitmap(&buffer);
                let diff = xor_bitmaps(&prev_frame, &bitmap);
                prev_frame = bitmap;

                let changes = get_change_runs(&diff);
                output_file.write_all(&get_frame_change_bytes(&changes)).unwrap();
            }
            Err(_) => break
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Input file required");
    }
    let input_filename = &args[1];

    fs::create_dir_all("output").unwrap();
    let mut output_file = File::create("output/output.2col").unwrap();
    let mut stream = ffmpeg_stream(input_filename);
    let frame_size = 640*360*3; // width * height * bytes_per_pixel

    run_pipeline(&mut stream, frame_size, &mut output_file);
}
