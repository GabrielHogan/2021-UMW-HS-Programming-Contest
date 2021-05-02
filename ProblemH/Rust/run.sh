#!/bin/bash

echo "Requires cargo! Please find this here: https://www.rust-lang.org/tools/install"
cd ProblemH
cargo clean
cargo build --release
cd ./target/release
echo "Running:"
./ProblemH