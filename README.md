# leq8

An 8-bit vitual machine written in Rust.

Features
  - Fully 8-bit; all pointers including the instruction pointer are a single byte, so the program data and RAM are both limited to 256B.
  - Graphical output using Raylib.
  - Assembly language and assembler.
  - 26 instructions for artihmetic, control flow and IO.

The `progs` folder contains two programs written in assembly for the virtual machine
- move.leq is a simple 2D movement program
- snake.leq is an implementation of the snake game
