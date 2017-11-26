# flex

[![Build Status](https://travis-ci.org/chris007de/flex.svg?branch=master)](https://travis-ci.org/chris007de/flex/builds)

This is an Open-Source implementation of the FLEX paging protocol encoder by DL1COM and DC1MIL.

This coder is based on US patent US55555183 and intended for non-commercial use only.
All patent-related information is property of their respective owners, if still applicable.

## Implementation

Current state of implementation:

- Speed: 1600
- Block Information Words
  - BIW 1
  - LocalID/Timezone
  - Day/Month/Year
  - Hour/Minute/Second
  - (Time and Date is currently hardcoded)
- Message Types:
  - AlphaNum
- Only "Short Address" CAPCODES supported
- No inter-frame fragmentation possible, messages which do not fit into the current are sent in the next frame.
Currently, it is possible to generate a FLEX bytestream from a JSON input file.

## Usage

```
flex --MODE -i INPUT -o OUTPUT

Options:
    Modes:
    --single        generate only frames for the messages given
    --hour          generate a full hour of frames
    --continuous    generate frames continuously to UDP

    -i, --input FILE.json
                        set input file (JSON)
    -o, --output FILE   set output file (FLEX bytestream)
```

## JSON Message Format

```JSON
[  
    {  
        "frame":0,
        "msgtype":"AlphaNum",
        "capcode":123123,
        "data":"test1"        
    },
    {  
        "frame":1,
        "msgtype":"AlphaNum",
        "capcode":123124,
        "data":"test2"
    }
]
```

### Examples

#### Operation Mode: single

Generate only the frames for the messages contained in the JSON file.

```Shell
flex --single -i messages.json -o messages.bin
```


#### Operation Mode: hour

Generate a full hour of FLEX frames, containing the messages from the JSON file in the respective frames.

```Shell
flex --hour -i messages.json -o messages.bin
```

#### Operation Mode: continuously

Generates frames permanently, each 1.875 seconds and sends them to *127.0.0.1:51337*.
There could be a GNU Radio UDP Source waiting to do modulate and send the frames.

Input for frames could be done using a named pipe, e.g.:

```Shell
mkfifo input.pipe
flex --continuous -i input.pipe

echo '[{"msgtype":"AlphaNum","capcode":123456,"data":"test","frame":1}]' > input.fifo
```


### Build instructions

```Shell
cargo build
```

### Test

1. Generate FLEX bytestream (see examples above)
2. Modulate bytestream using 2-FSK modulator (e.g. https://github.com/dl0muc/gr-flexpager)
3. Send directly to pager using your favorite SDR or dump audio to raw audio file and decode with multimon-ng 

    ```
    multimon-ng -t raw -a FLEX -v 3 dump.raw
    ```

## Usage of flexencoder crate

```Rust
// Create a message
let msg = Message::new(0, MessageType::AlphaNum, 0x123456, String::from("test")).unwrap();

// Create a frame
let mut frame = Frame::new(cycle_nr, frame_nr).unwrap();

// Check if there is still space left in frame
if msg.get_num_of_message_codewords().unwrap() > frame.space_left() {
    panic!("No space left in frame.");
    }

// Add message to frame
frame.add_message(&msg).unwrap();

// Get frame bytes to send them
let bytes = frame.get_bytes();
```