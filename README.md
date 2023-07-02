# fprop

## What is it?

A Rust program to show the properties of files/directories whose paths are provided as CLI arguments (these properties are a subset of those shown in e.g. the File Properties dialog in Windows).

## Why does it exist?

No particular reason yet - it's just a small exercise in Rust.

## Usage

If running the executable directly:

`fprop <path_to_file1> <path_to_file2> <path_to_file3> ...`

If running the program via `cargo`:

`cargo run --release -- <path_to_file1> <path_to_file2> <path_to_file3> ...`