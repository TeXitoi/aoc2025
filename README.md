# Advent of Code 2025

This repository contains my solutions for [Advent of Code
2025](https://adventofcode.com/2025/).

You can also find my [Advent of Code 2022
solutions](https://github.com/TeXitoi/aoc2022).

Goal is readable, simple and mostly clean.

Each day is solved in it’s dedicated file in the [src/bin](src/bin/)
directory.

Only a few dependencies are used:
* [anyhow](https://crates.io/crates/anyhow) for easy and simple error
  handling, in all the files.
* [microlp](https://crates.io/crates/microlp), a simple MILP solver in
  pure rust, to solve day 10 part 2.

# About my writing of these files

I am an experienced rust developer. I use rust since 2014 (so before
rust 1.0). You may know me for
[structopt](https://github.com/TeXitoi/structopt) or
[keyberon](https://github.com/TeXitoi/keyberon). I like to use
iterators, the `?` operator and prefer (a bit too much) short names to
comments.

I have solved these problems by doing some "dirty" things (as
`.clone()` abuse, copy and paste, unreadable mess, damn slow algorithm
running during lunch). Then I have cleaned them, and sometime improved
them. They all run in a reasonable time.

All these programs should solve any problem from the official site.
