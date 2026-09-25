# User Guide

The main types you will interact with are `Board`, `IntMove`, and `Piece`.

# Board

This is the class that represents the chess board.

The most important field of the class is `valid_moves`. It contains a `Vec<IntMove>` containing all legal moves for the current position.

To create a `Board`, you can use `Board::from_fen`. It takes a string containing the FEN representation of a position as input. The move count is not included in this FEN representation.

It returns a 'Result<Board, String>`.

`Board.get_int_board()` takes no input and returns a 2D array of integers representing the board.

An empty square is represented by `12`. White pieces are in the range `0-6` and black pieces are in the range `6-12`. To convert an index into a `Piece`, you can use `Piece::from_index`.

`Board.make_and_validate_move(int_move: IntMove)` takes an `IntMove` and returns a `Result<(), ()>`. If the move is invalid, it returns an error.

If the move is valid, the function updates the board state for you. This includes switching the turn and generating the valid moves for the new position.

`Board.get_board_representation()` takes no input and returns a String following the convention we agreed on for communication.

`Board.is_stalemate() -> bool` returns whether the current position is a stalemate.

`Board.is_checkmate() -> bool` returns whether the current position is checkmate.

# IntMove

This class represents a chess move and is how you interact with the board when making moves.

The class has 5 fields:

`sx: i8` — The starting x coordinate of the move.

`sy: i8` — The starting y coordinate of the move.

`ex: i8` — The ending x coordinate of the move.

`ey: i8` — The ending y coordinate of the move.

`promotion_piece: Option<Piece>` — The piece to promote to. This should be `None` if there is no promotion, otherwise it should be `Some(Piece)` to specify which piece the pawn promotes to.

All coordinates are 0-indexed, with `a8` being `(x: 0, y: 7)`.

`IntMove::parse_cords(&str) -> Result<(i8, i8), String>` takes a coordinate in chess notation, such as `e4`. If the conversion is successful, it returns a tuple containing the x coordinate first and the y coordinate second.

# Piece

The `Piece` enum consists of:

```text
Pawn
Rook
Knight
Bishop
Queen
King
```

`Piece::get_index(piece: Piece) -> usize` returns the piece as its integer representation.

`Piece::from_index(i: usize) -> Result<Piece, String>` returns the piece type corresponding to a `usize`. This is the reverse of `get_index`.

`Piece::get_type(c: &char) -> Result<Piece, String>` returns the piece type corresponding to a character.
