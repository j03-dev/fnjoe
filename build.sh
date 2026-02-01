#!/bin/bash

if [ -d "docs" ]; then
    rm -rf docs
fi

dx bundle --out-dir docs
mv docs/public/* docs
cp docs/index.html docs/404.html
rm -rf docs/public
