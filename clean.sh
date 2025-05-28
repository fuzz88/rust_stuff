#!/bin/bash

# Exit on error
set -e

# Check if an argument is provided
if [ -z "$1" ]; then
  echo "Usage: $0 <directory>"
  exit 1
fi

# Directory target
TARGET_DIR="$1"

# 1. Clean Cargo builds
find "./$TARGET_DIR/" -name Cargo.toml -execdir cargo clean \;

# 2. Remove all .gitignore files
find "./$TARGET_DIR/" -type f -name .gitignore -print -exec rm -v {} \;

# 3. Remove all .git directories
find "./$TARGET_DIR/" -type d -name .git -print -exec rm -rv {} \;

