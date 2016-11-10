#!/bin/bash
cargo build &&
cargo run &
sleep 1;
cargo test;
killall hms_api