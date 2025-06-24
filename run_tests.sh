#!/bin/bash

set -e

TERM_PURPLE='\033[0;35m'
TERM_BOLD='\033[1m'
TERM_RESET='\033[0m'

section() {
    echo -e "${TERM_PURPLE}${TERM_BOLD}=== $1 ===${TERM_RESET}"
}

# argument: "feature1,feature2,..."
test_only_features() {
    local features="$1"
    section "no default + $features"
    cargo test --lib --bins --tests --benches --no-default-features --features "$features" > /dev/null
}

test_additional_features() {
    local features="$1"
    section "default + $features"
    cargo test --lib --bins --tests --benches --features "$features" > /dev/null
}

test_only_features "no_std"
test_only_features "no_std,multithreaded"

test_additional_features "multithreaded"
test_additional_features "anyhow"
test_additional_features "anyhow,multithreaded"

section "All Default Features and doctests"
cargo test > /dev/null
