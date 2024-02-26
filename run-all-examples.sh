#!/bin/bash

for example in examples/*.rs
do
    NO_ANIMATION=1 cargo run --example "$(basename "${example%.rs}")"
done