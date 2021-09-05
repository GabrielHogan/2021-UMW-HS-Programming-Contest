echo "Requires cargo! Please find this here: https://www.rust-lang.org/tools/install"
cd ProblemI
cargo clean
cargo build --release
cd ./target/release
echo "Running:"
ProblemI
pause