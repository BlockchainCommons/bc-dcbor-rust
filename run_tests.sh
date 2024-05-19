#!/bin/bash

cargo test
cargo test --features multithreaded
cargo test --no-default-features --features no_std
cargo test --no-default-features --features no_std,multithreaded
