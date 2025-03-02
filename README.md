# pngAbe - Steganography Tool for PNG Images

**pngAbe** allows you to secretly encode messages into PNG files by embedding data in custom PNG chunks.

## Installation

Build from source with Cargo:

```bash
cargo build --release
cp target/release/pngAbe .
```

## Usage

### Commands

```
./pngAbe encode    -p PATH       -c CHUNK_TYPE    -m MESSAGE    # Encode a message into a PNG file
./pngAbe decode    -p PATH       -c CHUNK_TYPE                  # Decode a message from a PNG file
./pngAbe remove    -p PATH       -c CHUNK_TYPE                  # Remove a message chunk from a PNG file
./pngAbe print     -p PATH                                      # Print all chunks in a PNG file
```

### Parameters

- `PATH`: Path to the PNG file
- `CHUNK_TYPE`: A 4-character identifier for your message chunk (e.g., "HeLo", "RuSt")
- `MESSAGE`: The secret message to encode

## Security Note

Share your `CHUNK_TYPE` only with people you want to be able to decode your messages. The `CHUNK_TYPE` acts as a simple shared key.

## Examples

```bash
# Hide a message in dog.png with chunk type "RuSt"
./pngAbe encode -p dog.png -c RuSt -m "This is a secret message!"

# Retrieve the message
./pngAbe decode -p dog.png -c RuSt

# View all chunks in the file
./pngAbe print -p dog.png

# Remove the secret chunk
./pngAbe remove -p dog.png -c RuSt
```

## How It Works

pngAbe leverages the PNG file format's ability to contain custom chunks. These chunks can hold arbitrary data while maintaining compatibility with standard image viewers.

---

Enjoy hiding your secret messages! 🕵️‍♂️
