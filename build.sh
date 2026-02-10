#!/usr/bin/env sh
set -eu

cargo fmt
dx fmt
cargo clippy --all-targets -- -D warnings

if [ -d "docs" ]; then
    rm -rf docs
fi

dx bundle --release --verbose --base-path "fnjoe" --debug-symbols=false --out-dir docs
mv docs/public/* docs
cp docs/index.html docs/404.html
rm -rf docs/public
