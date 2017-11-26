use std::time;
use std::thread;
use std::sync::{Arc, Mutex};

extern crate chrono;
use chrono::prelude::*;

extern crate flexencoder;
use flexencoder::frame::Frame;
use flexencoder::message::*;

use std::fs::File;
use std::io::prelude::*;

extern crate getopts;
use getopts::Options;
use std::env;

extern crate bit_reverse;
use bit_reverse::ParallelReverse;

extern crate serde;
extern crate serde_json;

use std::net::UdpSocket;

use std::process::exit;

const CYCLES_PER_HOUR: u32 = 15;
const FRAMES_PER_CYCLE: u32 = 128;

enum OperationMode {
    Single,
    Hour,
    Continuous,
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} --MODE -i INPUT -o OUTPUT", program);
    print!("{}", opts.usage(&brief));
}

fn parse_arguments(args: &Vec<String>) -> Result<(String, String, OperationMode), &'static str> {
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("i", "input", "set input file (JSON)", "FILE.json");
    opts.optopt("o", "output", "set output file (FLEX bytestream)", "FILE");
    opts.optflag(
        "",
        "single",
        "MODE: generate only frames for the messages given",
    );
    opts.optflag("", "hour", "MODE: generate a full hour of frames");
    opts.optflag(
        "",
        "continuous",
        "MODE: generate frames continuously to UDP",
    );
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        exit(0);
    }

    let in_file;
    match matches.opt_str("i") {
        Some(x) => in_file = x,
        None => return Err("No input file given."),
    }

    let mut out_file = "/dev/null".to_string();
    if matches.opt_present("o") {
        out_file = matches.opt_str("o").unwrap();
    }

    let mode: OperationMode;
    if matches.opt_present("hour") {
        mode = OperationMode::Hour;
    } else if matches.opt_present("single") {
        mode = OperationMode::Single;
    } else if matches.opt_present("continuous") {
        mode = OperationMode::Continuous;
    } else {
        return Err("No operation mode given.");
    }

    return Ok((in_file, out_file, mode));
}

fn generate_hour(in_file: String, out_file: String) {
    let file = File::open(in_file).unwrap();
    let msg_vec: Vec<Message> = serde_json::from_reader(file).unwrap();

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
    write_frames_to_file(&frames, &out_file);
}

fn generate_single_frames(in_file: String, out_file: String) {
    let file = File::open(in_file).unwrap();
    let msg_vec: Vec<Message> = serde_json::from_reader(file).unwrap();

    let mut frames = Vec::new();
    for msg in &msg_vec {
        let mut frame = Frame::new(0, msg.frame).unwrap();
        frame.add_message(msg).unwrap();
        frames.push(frame);
        println!("added {:?}", msg);
    }
    write_frames_to_file(&frames, &out_file);
}

fn write_frames_to_file(frames: &Vec<Frame>, filename: &String) {
    let mut file = File::create(filename).unwrap();
    file.write_all(&frames_hton(frames)).unwrap();
}

fn frames_hton(frames: &Vec<Frame>) -> Vec<u8> {
    let mut reversed_bytes = Vec::new();
    for frame in frames {
        let bytes = frame.get_bytes();
        for byte in bytes {
            reversed_bytes.push(byte.swap_bits());
        }
    }
    return reversed_bytes;
}

fn generate_continuously(in_file: String) {
    let share_msg_input_0 = Arc::new(Mutex::new(Vec::new()));
    let share_msg_input_1 = share_msg_input_0.clone();

    let input_thread = thread::spawn(move || loop {
        let file = File::open(&in_file).unwrap();
        let mut msg_vec: Vec<Message> = serde_json::from_reader(file).unwrap();

        println!("Queuing message(s): {:?}", msg_vec);
        let mut shared = share_msg_input_0.lock().unwrap();
        shared.append(&mut msg_vec);
    });

    let socket = UdpSocket::bind("127.0.0.1:34254").expect("couldn't bind to address");

    let mut msg_vec: Vec<Message> = Vec::new();
    let encoder_thread = thread::spawn(move || loop {
        msg_vec.append(&mut share_msg_input_1.lock().unwrap());

        let dt = Utc::now();
        let (cycle_nr, frame_nr) = Frame::calculate_cycle_and_frame(dt.minute(), dt.second());
        println!(
            "Time: {}:{}, Cycle: {}, Frame: {} - Size of MsgQueue: {}",
            dt.minute(),
            dt.second(),
            cycle_nr,
            frame_nr,
            msg_vec.len()
        );

        let mut frame = Frame::new(cycle_nr, frame_nr).unwrap();
        for i in 0..msg_vec.len() {
            if msg_vec[i].frame == frame_nr {
                if msg_vec[i].get_num_of_message_codewords().unwrap() < frame.space_left() {
                    frame.add_message(&msg_vec[i]).unwrap();
                    msg_vec.remove(i);
                }
            }
        }

        socket
            .send_to(&frames_hton(&vec![frame]), "127.0.0.1:51337")
            .expect("couldn't send data");

        thread::sleep(time::Duration::from_millis(1875));
    });

    let _ = input_thread.join();
    let _ = encoder_thread.join();
}

fn main() {

    let args: Vec<String> = env::args().collect();
    let (in_file, out_file, mode) = parse_arguments(&args).unwrap();

    match mode {
        OperationMode::Single => generate_single_frames(in_file, out_file),
        OperationMode::Hour => generate_hour(in_file, out_file),
        OperationMode::Continuous => generate_continuously(in_file),
    }
}
