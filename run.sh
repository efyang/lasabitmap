#!/usr/bin/env bash
cargo run --release
echo "Optimizing png..."
optipng "map.png" -silent
echo "Done."
