 #!/bin/bash
echo "Making files"
cargo build
cargo readobj -- -all target/thumbv7em-none-eabi/debug/rust-example > check/all.sh
cargo readobj -- -S target/thumbv7em-none-eabi/debug/rust-example > check/sections.sh
cargo readobj -- -s target/thumbv7em-none-eabi/debug/rust-example > check/symbols.sh
cargo readobj -- -x .text target/thumbv7em-none-eabi/debug/rust-example > check/text.sh
