#!/usr/bin/env bash
declare -a arr=("28" "256" "512" "1024" "2048" "4096" "8192" "16384" "32768" "65536" "131072" "262144")

true > tests.log
cargo build --release

for i in "${arr[@]}"
do
    ./target/release/program-rust 1 ${i} 5 0 >> tests.log
    echo $'\n' >> tests.log
done
