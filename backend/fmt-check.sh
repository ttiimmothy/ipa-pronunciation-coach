#!/bin/bash
# Check formatting with 2-space indentation using nightly rustfmt
cargo +nightly fmt -- --check --config indent_style=Block,tab_spaces=2
