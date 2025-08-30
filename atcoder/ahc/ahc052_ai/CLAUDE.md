# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is an AtCoder Heuristic Contest (AHC052) solution implemented in Rust, specifically for the "Single Controller Multiple Robots" problem. The project is part of an experiment to solve competitive programming problems with LLM assistance using Claude Code constraints.

## Problem Context

- **Contest**: AHC052 - Single Controller Multiple Robots
- **Grid Size**: Fixed 30×30 grid with walls
- **Robots**: 10 robots controlled simultaneously with a single controller
- **Buttons**: 10 configurable buttons, each can assign different actions to each robot
- **Actions**: U (up), D (down), L (left), R (right), S (stay)
- **Goal**: Cover all grid cells with minimal operations (≤ 2N² = 1800 moves)
- **Scoring**: 3N² - T points if all cells covered, N² - R points otherwise

## Build and Run Commands

```bash
# Build for development
cargo build

# Build optimized version (recommended for contest submissions)
cargo build --release

# Run with sample input
cargo run --release < input.txt

# Run with debug output
cargo run < input.txt

# Test with multiple inputs (if input files exist)
for f in in/*.txt; do echo "=== $f ==="; cargo run --release < "$f"; done
```

## Code Architecture

### Main Structure
- `src/bin/a.rs`: Main solution file containing the complete implementation
- Input parsing handled by custom `Scanner` utility
- Problem modeled as `Game` struct with solver implementation

### Key Components
- **Game struct**: Represents the grid state, walls, and robot positions
- **Solver struct**: Contains the main algorithm logic
- **Scanner utility**: Handles input parsing with methods like `cin()`, `vec()`, `chars()`
- **Utility macros**: `min!`, `max!`, `clip!`, `trace!`, `put!`, `ndarray!` for debugging and output

### Input Format
- Line 1: N M K (grid size, robots, buttons) - always 30 10 10
- Lines 2-11: Robot initial positions (i_k, j_k)
- Lines 12-41: Vertical walls (v matrix)  
- Lines 42-70: Horizontal walls (h matrix)

### Output Format
- First 10 lines: Button configuration matrix (K×M chars)
- Following lines: Sequence of button presses (0-9)

## Development Notes

- The codebase uses a template structure common for competitive programming
- Debug output is controlled by `#[cfg(debug_assertions)]` 
- The `trace!` macro is useful for debugging during development
- Release builds use LTO and panic='abort' for optimization