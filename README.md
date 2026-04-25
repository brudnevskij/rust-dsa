# Algorithms Practice

Rust workspace with small algorithm exercises.

## Arrays crate

The `arrays` crate contains array-related exercises.


## Run all tests

From the workspace root:

```bash
cargo test
```

## Run tests for one crate

```bash
cargo test -p arrays
```

## Run tests for one module/file

For example:

```bash
cargo test -p arrays n_max
```

or:

```bash
cargo test -p arrays n_min
```
