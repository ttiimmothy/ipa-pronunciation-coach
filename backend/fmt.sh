#!/bin/bash
# Format code with 2-space indentation using nightly rustfmt
cargo +nightly fmt -- --config indent_style=Block,tab_spaces=2
