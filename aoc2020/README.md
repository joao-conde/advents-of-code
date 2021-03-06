# [Advent of Code 2020](https://adventofcode.com/2020)

## Why Rust

In university, I was taught [Scheme](https://plt-scheme.org/) as my first programming language. Despite not hating it, I wouldn't consider it my first choice for my daily job, let alone for systems programming. The second programming language I learned was [C++](https://www.cplusplus.com/).

After that, I did a lot of Python and JavaScript for web development and some Java for distributed systems and mobile applications. But C++ remained my language of choice for writing programs whose efficiency mattered. Examples of such are my [competitive programming exercises and contest problems](https://github.com/joao-conde/competitive-programming), where speed and fine-grained memory control was a necessity. I can't write Python or Java programs in this context because booting up their VMs alone would take too much time (and resources).

However, after programming for a while in C++, I ran into some classic problems with languages that give you fine-grained control over memory management (like C). **Issues like dangling pointers, use-after-frees, double frees, data races, and others. Rust introduces an ownership and borrowing memory model which completely eradicates these issues.** I don't know about you, but after knowing about this, how can I ever go back to C or C++?

<img src="https://rustacean.net/assets/cuddlyferris.png" width=50% height=50%>

## Running each solution

The following assumes you are at the project root level (same as `Cargo.toml` and this `README.md`).

To run the solution for a given day:

```bash
$ cargo run --bin day[01-25]
```

To run all the solutions:

```bash
$ cargo run --bin all
```
