# VByte Encoding and Decoding Library

This library provides efficient implementations for VByte (Variable Byte) encoding and decoding, a common technique used in information retrieval systems to compress integers.

## Features

- Efficient encoding and decoding of integers using the VByte algorithm.
- Supports the full range of `u32` integers.
- Simple and easy-to-use API.

## Installation

Add the following to your `Cargo.toml`:

```rust
[dependencies]
vbyte_rs = "0.1.0"
```

## Usage

Here's a basic example of how to use the library:

```rust
extern crate vbyte_rs;

fn main() {
    // Encoding
    let mut encoded: Vec<u8> = Vec::new();
    vbyte_rs::vbyte_encode(300, &mut encoded);
    println!("Encoded: {:?}", encoded);

    // Decoding
    let decoded = vbyte_rs::vbyte_decode(&encoded).unwrap();
    println!("Decoded: {}", decoded);
}
```

### API
`vbyte_encode(n: u32, output: &mut Vec<u8>)`
Encodes a u32 integer using the VByte algorithm and appends the encoded bytes to the provided output vector.

`vbyte_decode(bytes: &[u8]) -> Option<u32>`
Decodes a VByte encoded integer from the provided byte slice. Returns Some(u32) if decoding is successful, otherwise returns None.

### Testing
Run the tests using:

```bash
cargo test
```

### Benchmarking
Benchmark the encoding and decoding functions using:

```bash
cargo bench
```

### License
MIT License







