# KnO3 - A Rusty Knight Chess Engine

This project is a chess engine built in rust. It provides an engine for playing chess, and can be attached to any GUI.

## Primary Contributors
- [Cooper Hanson](https://github.com/chanson02/)
- [Abrahim Toutoungi](https://github.com/abrahimt)

## Feature List
- **Move Generation**: Efficient algorithms for generating legal chess moves for a player (not yet implemented)
- **Board Representation**: Chessboards are stored as a collection of bitmapped pieces. The Chessboard object comes with human readable representations in the form of the `print` method and `to_string` which returns a [FEN](https://www.chess.com/terms/fen-chess) string.
- **Rule Enforcement**: The engine can verify moves, making sure they align with standard chess rules. See `src/board/movement.rs#is_valid_move_for_piece`.
- **FEN Notation**: Support for [Forsyth-Edwards Notation](https://www.chess.com/terms/fen-chess) for describing chessboards. A FEN string can be passed from engine to GUI and vice-versa.

## Resources
- [Chess Programming Wiki](https://www.chessprogramming.org/Main_Page)
- [Forsyth-Edwards Notation](https://www.chess.com/terms/fen-chess)

## Coordinates

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
