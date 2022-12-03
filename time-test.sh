#!/bin/bash

set -e

cargo build
cargo build --release

time target/debug/advent-of-code
time target/release/advent-of-code

