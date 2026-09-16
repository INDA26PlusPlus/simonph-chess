mod board;
use board::Board;
use crate::{chessmove::BitMove,chessmove::IntMove, utils::Colour};
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
    
    let mut curplayer = Colour::White;
    loop{
        board.print_bitmasks();
        board.print_board();
        let mut inp:String = String::new();
        std::io::stdin().read_line(&mut inp).unwrap();
        let int_move = match IntMove::parse(inp){
            Ok(val) => val,
            Err(e) => {
                println!("{e}");
                continue;
            }
        };
        let possible_moves = board.get_all_moves(&curplayer);
        if !possible_moves.contains(&int_move){
            println!("not a valid move");
            continue;
        }
        let bit_move = BitMove::from_int_move(int_move,&board,&curplayer).unwrap();
        board.excecute_move(&bit_move);
        curplayer = match curplayer{
            Colour::White => Colour::Black,
            Colour::Black => Colour::White,
        };
    }
}
