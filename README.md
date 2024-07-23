# KnO3 - A Rusty Knight Chess Engine

This project is a chess engine built in rust. It provides an engine for playing chess, and can be attached to any GUI
## Primary Contributors
- [Cooper Hanson](https://github.com/chanson02/)
- [Abrahim Toutoungi](https://github.com/abrahimt)

## Usage
The engine can be interfaced through stdio with the help of the CLI.
`cargo run -- --help` to see all available options.
- **Example**: `cargo run -- -f "8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b - - 99 50" --get-moves f7 --show`


## Architecture
The engine creates a new `GameState` based on the FEN string that was passed in.

The engine is a library that is completely detatched from the CLI. This would allow anyone to hook up just the engine to an existing chess application and be able to interface with it. The CLI and potentially a server could be used for cross communication between different languages.

### Board Representation

Each rank-file position can be represented as a 0-63 decimal number. The positions are shown in the table below. A 64bit bitmap can be used to represent an entire board. If a piece is at position H7 (63), the first bit (64) will be enabled.

| R | A | B | C | D | E | F | G | H |
|---|---|---|---|---|---|---|---|---|
| 7 | 56| 57| 58| 59| 60| 61| 62| 63|
| 6 | 48| 49| 50| 51| 52| 53| 54| 55|
| 5 | 40| 41| 42| 43| 44| 45| 46| 47|
| 4 | 32| 33| 34| 35| 36| 37| 38| 39|
| 3 | 24| 25| 26| 27| 28| 29| 30| 31|
| 2 | 16| 17| 18| 19| 20| 21| 22| 23|
| 1 |  8|  9| 10| 11| 12| 13| 14| 15|
| 0 |  0|  1|  2|  3|  4|  5|  6|  7|
