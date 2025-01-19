Conway's Game of Life
=====================

This is a tool to play around with a visualisation of Conway's Game of Life. Conway's Game of Life (a.k.a. Game of Life) is a
cellular automaton devised by the British mathematician John Horton Conway.

The game plays out over a two-dimensional grid where cells can be in one of two possible states: dead or alive. Once started, the game plays itself out according to the
following rules:

- Any live cell with fewer than two live neighbours dies, as if by underpopulation.
- Any live cell with two or three live neighbours lives on to the next generation.
- Any live cell with more than three live neighbours dies, as if by overpopulation.
- Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.

https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life

Build Instructions
==================
Build the game with `cargo build`. Then run the binary that is placed in `target/debug`.
```shell
Usage: game-of-life [OPTIONS] --input <input>

Options:
-i, --input <input>    List of cells that should be alive when the simulation starts. Concatenated by a semicolon. E.g. 1,2;3,4 maps to the cells x1,y2 and x3,y4 to be alive when the simulation starts.
--width <width>    Width of the viewport in the terminal [default: 120]
--height <height>  Height of the viewport in the terminal [default: 60]
-h, --help             Print help
-V, --version          Print version
```

Example input: `./target/debug/game-of-life --width 100 --height 60 -i "0,2;1,2;2,2;2,1;1,0"`