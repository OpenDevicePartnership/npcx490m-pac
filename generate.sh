#!/bin/bash
set -e

svdtools patch patch/NPCX490M.yaml
svd2rust -i svd/NPCX490M.svd.patched --reexport-interrupt --reexport-core-peripherals --ignore-groups --impl-defmt defmt --impl-debug --impl-debug-feature debug
rm -r src/*
form -i lib.rs -o src
rm lib.rs
cargo fmt
