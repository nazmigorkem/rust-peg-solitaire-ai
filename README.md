## Peg Solitaire Solution in Rust

### Requirements

In order to run this repository you have to have cargo and rust compiler installed on your computer. You can install it form [here]().

This solution only works on Linux operating systems.

### Commands

In order to specify algorithm and time limit use the following command via replacing the \<values\>.

-   `cargo r -r -- -s <search-algorithm> -t <time-limit-in-minutes>`

### Available Options

-   `bfs` for breadth-first search
-   `dfs` for depth-first search
-   `random-dfs` for random node selection depth-first search
-   `iterative-dfs` for iterative deepening search
-   `heuristic-dfs` for heuristic node selection depth-first search

### Examples

-   `cargo r -r -- -s bfs -t 60`
-   `cargo r -r -- -s dfs -t 60`
-   `cargo r -r -- -s iterative-dfs -t 60`
-   `cargo r -r -- -s random-dfs -t 60`
-   `cargo r -r -- -s heuristic-dfs -t 60`
