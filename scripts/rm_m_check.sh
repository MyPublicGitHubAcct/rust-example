 #!/bin/bash
echo "Removing files"
rm check/all.sh
rm check/sections.sh
rm check/symbols.sh
rm check/text.sh
cargo clean
