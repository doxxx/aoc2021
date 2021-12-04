# Advent of Code 2021

This is my collection of apps used to solve the [Advent of Code 2021](https://adventofcode.com/2021) puzzles. They are written in Rust 2021 Edition.

The root is a Cargo workspace with subfolders containing an application for each day. Each app accepts its input data on stdin. To run a particular app, change to the appropriate `dayN` folder and execute the equivalent of the following for your shell:

```sh
cat input | cargo run --release
```
