#!/bin/bash

echo "Requires cargo! Please find this here: https://www.rust-lang.org/tools/install"
cd ProblemA
cargo clean
cargo build --release
cd ./target/release
./ProblemA