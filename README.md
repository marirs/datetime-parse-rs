Date Time parser
=================
[![Build Status](https://travis-ci.com/marirs/datetime-parse-rs.svg?branch=main)](https://travis-ci.com/marirs/datetime-parse-rs)
![GitHub](https://img.shields.io/github/license/marirs/datetime-parse-rs)

Parse various different date/time formats to a standard RFC 3339 format as chrono DateTime FixedOffset.

*Note*
- If date/time does <u>NOT</u> have `year`; `current year` is added
- if date/time does <u>NOT</u> have `time-zone` info; `Local time-zone info` is added

### Usage
```toml
[dependencies]
datetime_parser = "0.0.1-alpha"
```

and

```rust
use datetime_parse::DateTimeFixedOffset;

fn main() {
    let date_str = "Mon, 6 Jul 1970 15:30:00 PDT";
    let result = date_str.parse::<DateTimeFixedOffset>();
    assert!(result.is_ok());
    match result {
        Ok(parsed) => println!("{} => {:?}", date_str, parsed.0),
        Err(e) => println!("Error: {}", e)
    }
}
```

### Running the example
```bash
cargo run --example parse
```

### Requirements

- Rust 1.51+

### Contribution

Feel free to add more formats that you see, which is not present in the library.

---
License: MIT