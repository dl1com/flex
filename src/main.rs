extern crate flex;
use flex::frame::Frame;
use flex::message::*;

use std::fs::File;
use std::io::prelude::*;

extern crate getopts;
use getopts::Options;
use std::env;

extern crate bit_reverse;
use bit_reverse::ParallelReverse;

extern crate serde;
extern crate serde_json;

use std::process::exit;

const CYCLES_PER_HOUR : u32 = 15;
const FRAMES_PER_CYCLE: u32 = 128;

enum OperationMode {
    Single,
    Hour
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} --MODE -i INPUT -o OUTPUT", program);
    print!("{}", opts.usage(&brief));
}

fn parse_arguments(args: &Vec<String>) -> Result<(String,String,OperationMode),&'static str> {
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("i", "input","set input file (JSON)", "FILE.json");
    opts.optopt("o", "output","set output file (FLEX bytestream)", "FILE");
    opts.optflag("", "single","MODE: generate only frames for the messages given");
    opts.optflag("", "hour","MODE: generate a full hour of frames");
    opts.optflag("h", "help", "print this help menu");    
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        exit(0);
    }

    let in_file;
    match matches.opt_str("i") {
        Some(x) => in_file = x,
        None    => return Err("No input file given."),
    }

    let out_file;
    match matches.opt_str("o") {
        Some(x) => out_file = x,
        None    => return Err("No output file given."),
    }

    let mode: OperationMode;
    if matches.opt_present("hour") {
        mode = OperationMode::Hour;
    }
    else if matches.opt_present("single") {
        mode = OperationMode::Single;
    }
    else
    {
        return Err("No operation mode given.")
    }

    return Ok((in_file,out_file,mode));
}

fn generate_hour(msg_vec: Vec<Message>) -> Vec<Frame> {
    let mut frames = Vec::new();        
    for cycle_nr in 0..CYCLES_PER_HOUR {
        for frame_nr in 0..FRAMES_PER_CYCLE {
            let mut frame = Frame::new(cycle_nr, frame_nr).unwrap();
            for msg in &msg_vec {
                if msg.frame == frame_nr {
                    println!("added {:?}", msg);
                    frame.add_message(msg).unwrap();
                }
            }
            frames.push(frame);
        }
    }
    return frames;
}

fn generate_single_frames(msg_vec: Vec<Message>) -> Vec<Frame> {
    let mut frames = Vec::new();
    for msg in &msg_vec {
        let mut frame = Frame::new(0, msg.frame).unwrap();
        frame.add_message(msg).unwrap();
        frames.push(frame);
        println!("added {:?}", msg);
    }    
    return frames;
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let (in_file,out_file,mode) = parse_arguments(&args).unwrap();
    
    let file = File::open(in_file).unwrap();
    let msg_vec: Vec<Message> = serde_json::from_reader(file).unwrap();

    let frames;
    match mode {
        OperationMode::Single => frames = generate_single_frames(msg_vec),
        OperationMode::Hour => frames = generate_hour(msg_vec)
    }

    let mut file = File::create(out_file).unwrap();
    for frame in frames {
        let bytes = frame.get_bytes();
        let mut rotated_bytes = Vec::new();
        for byte in bytes {
            rotated_bytes.push(byte.swap_bits());
        }        
        file.write_all(&rotated_bytes).unwrap();
    }
}
