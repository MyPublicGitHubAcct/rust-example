 #!/bin/bash
echo "-- cargo build & mkdir check --"
echo "==============================="

cargo build
mkdir check

echo "-- make files all, sections, symbols, text --"
echo "============================================="

echo "cargo readobj -- -all target/thumbv7em-none-eabi/debug/rust-example > check/all.sh" > check/all.sh
cargo readobj -- -all target/thumbv7em-none-eabi/debug/rust-example >> check/all.sh

echo "cargo readobj -- -S target/thumbv7em-none-eabi/debug/rust-example > check/sections.sh" > check/sections.sh
cargo readobj -- -S target/thumbv7em-none-eabi/debug/rust-example >> check/sections.sh

echo "cargo readobj -- -s target/thumbv7em-none-eabi/debug/rust-example > check/symbols.sh" > check/symbols.sh
cargo readobj -- -s target/thumbv7em-none-eabi/debug/rust-example >> check/symbols.sh

echo "cargo readobj -- -x .text target/thumbv7em-none-eabi/debug/rust-example > check/text.sh" > check/text.sh
cargo readobj -- -x .text target/thumbv7em-none-eabi/debug/rust-example >> check/text.sh
