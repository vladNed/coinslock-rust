#!/bin/sh

echo "Starting the application"

RUST_LOG=debug cargo watch -x "run --bin coinslock"
