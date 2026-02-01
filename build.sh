#!/bin/bash
set -e

if [ -d "docs" ]; then
  rm -rf docs
fi

dx bundle --out-dir docs

mv docs/public/* docs
rm -rf docs/public
