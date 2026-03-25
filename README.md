# LeetCode Solutions in Rust

My solutions to LeetCode problems, written in Rust.

## Structure

Solutions are organized by topic. Each topic is a module under `src/`, and each problem is a separate file within its topic.

```
src/
├── lib.rs
├── array/
│   ├── mod.rs
│   ├── two_sum.rs
│   └── ...
├── linked_list/
│   ├── mod.rs
│   └── add_two_numbers.rs
├── string/
├── tree/
├── graph/
├── dynamic_programming/
├── binary_search/
└── ...
```

## Usage

Run all tests:

```sh
cargo test
```

Run tests for a specific topic:

```sh
cargo test array
```

Run tests for a specific problem:

```sh
cargo test two_sum
```
