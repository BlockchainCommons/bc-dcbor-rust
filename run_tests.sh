#!/bin/bash

set -e

cargo test
cargo test --features multithreaded
cargo test --no-default-features --features no_std
cargo test --no-default-features --features no_std,multithreaded
