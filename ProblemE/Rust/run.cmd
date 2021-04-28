echo "Requires cargo! Please find this here: https://www.rust-lang.org/tools/install"
cd ProblemE
cargo clean
cargo build --release
cd ./target/release
echo "Running:"
ProblemE
pause