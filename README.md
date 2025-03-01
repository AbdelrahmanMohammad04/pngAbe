Encode your secret messages into PNGs with pngAbe!

First, run cargo build --release && cp target/release/pngAbe .

Commands:
./pngAbe encode -p PATH -c CHUNK_TYPE -m MESSAGE
./pngAbe decode -p PATH -c CHUNK_TYPE
./pngAbe remove -p PATH -c CHUNK_TYPE
./pngAbe print  -p PATH

CHUNK_TYPE is a 4 character string, an example of a valid CHUNK_TYPE could be "HeLo"
After coming up with your CHUNK_TYPE, make sure to share it with those you want to be able to decode your message.

Enjoy! :D