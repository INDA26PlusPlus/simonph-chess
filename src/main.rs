use chesslib::Board;
use chesslib::IntMove;

fn main() {
    let mut board = match Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w -"){
        Ok(val) => val,
        Err(e) => {
            println!("{e}");
            return;
        }
    };
    
    //println!("{}",board.perft(4));
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
        let int_move = match IntMove::parse(&inp){
            Ok(val) => val,
            Err(e) => {
                println!("{e}");
                continue;
            }
        };

        match board.make_and_validate_move(int_move){
            Err(e) => println!("{e}"),
            Ok(()) => {},
        }
    }
}
