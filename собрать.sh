#!/bin/sh

set -xe

mkdir -p ./сборка/
rustc -o ./сборка/хуяк -g исходники/хуяк.rs
