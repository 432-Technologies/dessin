#!/bin/bash

set -e

for example in examples/*.rs
do
    NO_ANIMATION=1 NO_ICED=1 cargo run --example "$(basename "${example%.rs}")"
done
