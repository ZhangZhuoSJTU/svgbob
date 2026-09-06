#!/bin/bash
# ProgramBench-style build: must produce ./executable at the repo root.
set -e
cd "$(dirname "$0")"
cargo build --release -p svgbob_cli
cp target/release/svgbob_cli executable
