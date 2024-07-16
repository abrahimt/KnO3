# KnO3 - A Rusty Knight Chess Engine

This project is a chess engine built in rust. It provides an engine for playing chess, and can be attached to any GUI
## Primary Contributors
- [Cooper Hanson](https://github.com/chanson02/)
- [Abrahim Toutoungi](https://github.com/abrahimt)

## Architecture
An application can interface with the engine through the CLI by providing a FEN string. From there, the engine can output different actions.

### Board Representation

Each rank-file position can be represented as a 0-63 decimal number. The positions are shown in the table below. A 64bit bitmap can be used to represent an entire board. If a piece is at position H7 (63), the first bit (64) will be enabled.

| R | A | B | C | D | E | F | G | H |
|---|---|---|---|---|---|---|---|---|
| 8 | 56| 57| 58| 59| 60| 61| 62| 63|
| 7 | 48| 49| 50| 51| 52| 53| 54| 55|
| 6 | 40| 41| 42| 43| 44| 45| 46| 47|
| 5 | 32| 33| 34| 35| 36| 37| 38| 39|
| 4 | 24| 25| 26| 27| 28| 29| 30| 31|
| 3 | 16| 17| 18| 19| 20| 21| 22| 23|
| 2 |  8|  9| 10| 11| 12| 13| 14| 15|
| 1 |  0|  1|  2|  3|  4|  5|  6|  7|
