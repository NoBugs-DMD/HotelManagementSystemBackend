#!/bin/bash
cargo build &&
cargo run &
while [[ ! `pidof hms_api` ]]; do sleep 1; done
cargo test;
result=$?;
killall hms_api;
exit $result
