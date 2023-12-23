#!/bin/bash

#cargo test           $@ -- -Zunstable-options --report-time
cargo test --release $@ -- -Zunstable-options --report-time
