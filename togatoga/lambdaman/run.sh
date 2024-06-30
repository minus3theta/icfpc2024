#!/bin/bash
echo "Building..."
cargo build --release
# filter ../../data/lambdaman{d}.in
find ../../data -name "lambdaman*.in" | while read f; do
    # skip lambdaman21.in
    if [[ $f == *"lambdaman21.in"* ]]; then
        continue
    fi
    echo "Solving $f..."
    ./target/release/lambdaman --input $f --output ${f%.in}.toga.beam.out -g 1
    echo "Done and Saved to ${f%.in}.toga.beam.out"    
    output=${f%.in}.toga.beam.out
    filename=$(basename -- "$output")
    pushd ../../
    cargo run --bin icfpc_cli submit --output data/lambdaman/${filename} --task lambdaman
    popd
    sleep 5
done
