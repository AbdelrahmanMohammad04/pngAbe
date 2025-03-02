Encode your secret messages into PNGs with pngAbe!
<br>
First, run cargo build --release && cp target/release/pngAbe .
<br>
Commands:
<br>
./pngAbe encode -p PATH -c CHUNK_TYPE -m MESSAGE
<br>
./pngAbe decode -p PATH -c CHUNK_TYPE
<br>
./pngAbe remove -p PATH -c CHUNK_TYPE
<br>
./pngAbe print  -p PATH
<br>

CHUNK_TYPE is a 4 character string, an example of a valid CHUNK_TYPE could be "HeLo"
<br>
After coming up with your CHUNK_TYPE, make sure to share it with those you want to be able to decode your message.
<br>
Enjoy! :D
