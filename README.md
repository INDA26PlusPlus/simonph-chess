## User Guide:

The main classes are board and IntMove.

# Board
This is the class that represents the board.

Most important field of the class is valid_moves. It has a Vector of IntMoves containing all the legal moves at the time.

To create a Board you can run the Board::from_fen it takes in a string as input the fen for the position. movecount is not included in this fen representation. It will return a result<Board,String>

Board.get_int_board(). This takes no input and returns a 2d array of integers representing the board. An empty square is 12. White pieces are in the range 0-6 and black are 6-12. To get the piece type you can run the Piece::from_index function.

Board.make_and_validate_move(int_move: IntMove) the function takes in an IntMove. and returns a Result<(),()> If the move is invalid it will return an error. It will handle switching the state for you. By switching the turn bool aswell as generating valid moves.

board.is_stalemate() -> bool. The function returns if the position is stalemate.
board.is_checkmate() -> bool. The function returns if the position is checkmate.




The most important features that you will probably use will be in the board and int_move class


Int_move is a class that stores a move as a start coordinate end coordinate and promotion piece.


With an intmove you can perform moves on the chess board with make_and_validate_move.


You can also get all the valid intmoves from the field Board.valid_moves.


To get a nice representation of the board you may use Board.get_int_board and that will return a 2d array of ints representing the pieces
