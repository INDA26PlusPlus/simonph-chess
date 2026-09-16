use crate::board::Board;
use crate::chessmove::IntMove;
use crate::utils::Colour;
#[derive(PartialEq,Copy,Clone)]
pub enum Piece{
    Pawn,
    Rook,
    Knight,
    Bishop,
    King,
    Queen,
}
const DIRECTIONS: [(i8, i8); 8] = [
    (1,0),(0,1),(-1,0),(0,-1),(1,1),(-1,1),(-1,-1),(1,-1)
];
impl Piece{
    pub fn get_index(piece:Piece) -> usize{
        match piece{
            Piece::Pawn => 0,
            Piece::Rook => 1,
            Piece::Knight => 2,
            Piece::Bishop => 3,
            Piece::Queen => 4,
            Piece::King => 5,
            
        }
    }
    pub fn from_index(i:usize) -> Result<Piece,String>{
        match i{
            0 => Ok(Piece::Pawn),
            1 => Ok(Piece::Rook),
            2 => Ok(Piece::Knight),
            3 => Ok(Piece::Bishop),
            4 => Ok(Piece::Queen),
            5 => Ok(Piece::King),
            _ => Err(String::from("not correct")),
        }
    }
    pub fn get_type(c:&char) -> Result<Piece,String>{
        match c.to_ascii_lowercase(){
            'p' => Ok(Piece::Pawn),
            'r' => Ok(Piece::Rook),
            'n' => Ok(Piece::Knight),
            'b' => Ok(Piece::Bishop),
            'q' => Ok(Piece::Queen),
            'k' => Ok(Piece::King),
            _ => Err(format!("{c} is not a piece"))
        }
    }
    fn generate_from_direction(sx:i8, sy:i8, dx:i8, dy:i8, board:&Board,colour:&Colour)->Vec<IntMove>{
        let mut cx = sx + dx;
        let mut cy = sy + dy;
        let mut ret:Vec<IntMove> = Vec::new();
        while Board::is_inside_board(cx, cy){
            if !board.is_empty(cx, cy){
                match board.get_piece(Board::getpos(cx,cy), &colour){
                    Some(_) => {},
                    None => {
                        ret.push(IntMove{sx:sx,sy:sy,ex:cx,ey:cy,promotion_piece:None});
                    }
                }
                break;
            }
            ret.push(IntMove{sx:sx,sy:sy,ex:cx,ey:cy,promotion_piece:None});
            cx += dx;
            cy += dy;
        }
        ret
    }
    fn generate_bishop(sx:i8,sy:i8,board:&Board, colour:&Colour)->Vec<IntMove>{
        let mut ret:Vec<IntMove> = Vec::new();
        for i in 4usize..8{
            ret.extend(Piece::generate_from_direction(sx,sy,DIRECTIONS[i].0,DIRECTIONS[i].1,&board,&colour));
        }
        ret
    }
    fn generate_rook(sx:i8,sy:i8,board:&Board, colour:&Colour)->Vec<IntMove>{
        let mut ret:Vec<IntMove> = Vec::new();
        for i in 0usize..4{
            ret.extend(Piece::generate_from_direction(sx,sy,DIRECTIONS[i].0,DIRECTIONS[i].1,&board,&colour));
        }
        ret
    }
    fn generate_queen(sx:i8,sy:i8,board:&Board, colour:&Colour)->Vec<IntMove>{
        let mut ret:Vec<IntMove> = Vec::new();
        for i in 0usize..8{
            ret.extend(Piece::generate_from_direction(sx,sy,DIRECTIONS[i].0,DIRECTIONS[i].1,&board,&colour));
        }
        ret
    }
    fn generate_pawn(sx:i8,sy:i8,board:&Board,colour:&Colour)->Vec<IntMove>{
        let mut ret:Vec<IntMove> = Vec::new();
        let atstart = match colour{
            Colour::Black => sy == 6,
            Colour::White => sy == 1,
        };
        let dy:i8 = match colour{
            Colour::Black=>-1,
            Colour::White=>1,
        };
        if board.is_empty(sx,sy+dy){
            ret.push(IntMove { sx, sy, ex:sx, ey: sy + dy, promotion_piece: None});
            if atstart && board.is_empty(sx, sy + 2*dy){
                ret.push(IntMove { sx, sy, ex:sx, ey: sy + 2*dy, promotion_piece: None});
            }
        }
        if Board::is_inside_board(sx-1, sy + dy){
            if board.get_piece(Board::getpos(sx-1,sy+dy),&colour.opposite()).is_some(){
                ret.push(IntMove { sx, sy, ex:sx-1, ey: sy + dy, promotion_piece: None});
            }
            if board.can_enpasant(Board::getpos(sx-1, sy + dy)){
                ret.push(IntMove { sx, sy, ex:sx-1, ey: sy + dy, promotion_piece: None});
            }
        }
        if Board::is_inside_board(sx + 1, sy + dy){
            if board.get_piece(Board::getpos(sx+1,sy+dy),&colour.opposite()).is_some(){
                ret.push(IntMove { sx, sy, ex:sx+1, ey: sy + dy, promotion_piece: None});
            }
            if board.can_enpasant(Board::getpos(sx + 1, sy + dy)){
                ret.push(IntMove { sx, sy, ex:sx+1, ey: sy + dy, promotion_piece: None});
            }
        }
        ret
    }
    fn generate_king(sx:i8,sy:i8,board:&Board,colour:&Colour)->Vec<IntMove>{
        let mut ret : Vec<IntMove> = Vec::new();
        for (dx,dy) in DIRECTIONS{
            if Board::is_inside_board(sx+dx, sy+dy) && board.get_piece(Board::getpos(sx+dx,sy+dy), colour).is_none(){
                ret.push(IntMove{sx:sx,sy:sy,ex:sx + dx,ey:sy+dy,promotion_piece:None});
            }
        }
        ret
    }
    fn generate_knight(sx:i8,sy:i8,board:&Board,colour:&Colour) -> Vec<IntMove>{
        let mut ret : Vec<IntMove> = Vec::new();
        let mut dx:i8 = 2;
        let mut dy:i8 = 1;
        for i in 4usize..8{
            let cx = sx + dx*DIRECTIONS[i].0;
            let cy = sy + dy*DIRECTIONS[i].1;
            if Board::is_inside_board(cx, cy) && board.get_piece(Board::getpos(cx, cy), colour).is_none(){
                ret.push(IntMove{sx,sy,ex:cx,ey:cy,promotion_piece:None});
            }
        }
        dx = 1;
        dy = 2;
        for i in 4usize..8{
            let cx = sx + dx*DIRECTIONS[i].0;
            let cy = sy + dy*DIRECTIONS[i].1;
            if Board::is_inside_board(cx, cy) && board.get_piece(Board::getpos(cx, cy), colour).is_none(){
                ret.push(IntMove{sx,sy,ex:cx,ey:cy,promotion_piece:None});
            }
        }
        ret
    }
    pub fn generate_moves(sx:i8,sy:i8,board:&Board, colour:&Colour, piece:&Piece)->Vec<IntMove>{
        match piece{
            Piece::Pawn => Piece::generate_pawn(sx, sy, board, colour),
            Piece::Rook => Piece::generate_rook(sx, sy, board, colour),
            Piece::Knight => Piece::generate_knight(sx, sy, board, colour),
            Piece::Bishop => Piece::generate_bishop(sx, sy, board, colour),
            Piece::Queen => Piece::generate_queen(sx, sy, board, colour),
            Piece::King => Piece::generate_king(sx, sy, board, colour),
        }
    }
}