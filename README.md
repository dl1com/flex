# flex

[![Build Status](https://travis-ci.org/chris007de/flex.svg?branch=master)](https://travis-ci.org/chris007de/flex/builds)

This is an Open-Source implementation of the FLEX paging protocol encoder.

The coder is based on US patent US55555183 and intended for non-commercial use only.
All patent-related information is property of their respective owners, if applicable.

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
    -i, --input FILE.json
                        set input file (JSON)
    -o, --output FILE   set output file (FLEX bytestream)
        --single        generate only frames for the messages given
        --hour          generate a full hour of frames
    -h, --help          print this help menu
```

## JSON Message Format

```
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

```
flex --single -i messages.json -o messages.bin
```


#### Operation Mode: hour

Generate a full hour of FLEX frames, containing the messages from the JSON file in the respective frames.

```
flex --hour -i messages.json -o messages.bin
```

### Build instructions

```
cargo build
```

### Test

1. Generate FLEX bytestream (see examples above)
2. Modulate bytestream using 2-FSK modulator (e.g. https://github.com/dl0muc/gr-flexpager)
3. Send directly to pager using your favorite SDR or dump audio to raw audio file and decode with multimon-ng 

    ```
    multimon-ng -t raw -a FLEX -v 3 dump.raw
    ```

