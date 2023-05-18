# libstark2rust

## Deps
Need to install cargo [make](https://github.com/sagiegurari/cargo-make#installation)
```bash
cargo install --force cargo-make
```

## Build
First need to initialize libSTARK dependency
```bash
cargo make init
```
Build
```bash
cargo build
```
Run
```
cargo run
```

Expected output:
```
Permutations: [0, 1, 2, 3, 4, 6, 5] (size: 7)
Assignment: [[1, 2, 3], [2, 3, 4], [9, 8, 7], [12, 3, 4], [0, 0, 7], [1, 1, 1], [2, 2, 2]] (size: 7)
```
