#!/bin/bash

set -xe

find ./ -name Cargo.toml -execdir cargo clean \;
rm -v $(find . -type f -name .gitignore)
rm -rv $(find . -type d -name .git)

