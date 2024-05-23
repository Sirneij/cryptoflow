#!/usr/bin/env bash
cargo build --release
cargo install sqlx-cli --no-default-features --features native-tls,postgres