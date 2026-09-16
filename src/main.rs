mod board;
use board::Board;
use crate::{chessmove::IntMove};
mod piece;
mod utils;
mod chessmove;


fn main() {
    let mut board = match Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR"){
        Ok(val) => val,
        Err(e) => {
            println!("{e}");
            return;
        }
    };
    loop{
        board.print_board();
        if board.is_checkmate(){
            println!("CHECKMATE!");
            break;
        }
        if board.is_stalemate(){
            println!("DRAW :(");
            break;
        }
        let mut inp:String = String::new();
        std::io::stdin().read_line(&mut inp).unwrap();
        let int_move = match IntMove::parse(inp){
            Ok(val) => val,
            Err(e) => {
                println!("{e}");
                continue;
            }
        };

        match board.make_and_validate_move(int_move){
            Some(e) => println!("{e}"),
            None => {},
        }
    }
}
