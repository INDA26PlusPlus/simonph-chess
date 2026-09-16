use crate::piece::Piece;
use crate::utils::Colour;
use crate::chessmove::BitMove;
use crate::chessmove::IntMove;
pub struct Board {
    turn:Colour,
    white_all: u64,
    black_all: u64,
    white_pieces: [u64; 6],
    black_pieces: [u64; 6],
    pub enpasant: u64,
    pub castle_rights: u8,
}

impl Board {
    pub fn getpos(x:i8, y:i8) -> i8{
        y*8 + x
    }
    pub fn getbit(x:i8, y:i8) -> u64{
        return 1u64 << Board::getpos(x,y);
    }
    fn getnormal(i:i8) -> (i8,i8){
        let x = i%8;
        let y = i/8;
        (x,y)
    }
    fn frombitpos(x:u64) -> (i8,i8){
        for i in 0..64{
            if x < (1u64 << i){
                return Board::getnormal(i-1);
            }
        }
        return (-1,-1);
    }
    pub fn can_enpasant(&self, i:i8)->bool{
        return (self.enpasant & (1u64 << i)) != 0;
    }
    pub fn get_piece(&self,i:i8,colour:&Colour)->Option<Piece>{
        match colour{
            Colour::White =>{
                if self.white_all & (1u64 << i) == 0{
                    return None;
                }
                else{
                    for pi in 0usize..6{
                        if self.white_pieces[pi] & (1u64 << i) != 0{
                            return Some(Piece::from_index(pi).unwrap());
                        }
                    }
                }
            },
            Colour::Black =>{
                if self.black_all & (1u64 << i) == 0{
                    return None;
                }
                else{
                    for pi in 0usize..6{
                        if self.black_pieces[pi] & (1u64 << i) != 0{
                            return Some(Piece::from_index(pi).unwrap());
                        }
                    }
                }
            }
        }
        return None;
    }
    pub fn new() -> Board {
        Board {
            turn : Colour::White,
            white_all: 0,
            black_all: 0,
            white_pieces: [0; 6],
            black_pieces: [0; 6],
            enpasant: 0,
            castle_rights: 15,
        }
    }
    pub fn create_piece(&mut self, piece_type: Piece, colour: Colour, x: i8, y: i8) {
        let i = Piece::get_index(piece_type);
        let j = Board::getpos(x, y);
        match colour {
            Colour::Black => {
                self.black_pieces[i] |= 1 << j;
                self.black_all |= 1 << j;
            }
            Colour::White => {
                self.white_pieces[i] |= 1 << j;
                self.white_all |= 1 << j;
            }
        }
    }
    pub fn from_fen(fen: &str) -> Result<Board, String> {
        let mut x: i8 = 0;
        let mut y: i8 = 7;
        let mut board = Board::new();

        for c in fen.chars() {
            match c {
                '/' => {
                    y -= 1;
                    x = 0;
                }
                '1'..='8' => {
                    x += c.to_digit(10).unwrap() as i8;
                }
                _ => {
                    let t: Piece = match Piece::get_type(&c) {
                        Ok(val) => val,
                        Err(e) => return Err(e),
                    };
                    let colour = match c.is_lowercase() {
                        true => Colour::Black,
                        false => Colour::White,
                    };
                    board.create_piece(t, colour, x, y);
                    x += 1;
                }
            }

            // if y < 0 || x >= 8{
            //     return Err(String::from("Index out of bounds"));
            // }
        }
        return Ok(board);
    }
    #[allow(dead_code)]
    fn print_bitmask(mask: &u64) {
        for i in (0..8).rev() {
            for j in 0..8 {
                let bit: i8 = match mask & (1 << Board::getpos(j, i)) {
                    0 => 0,
                    _ => 1,
                };
                print!("{bit} ")
            }
            println!();
        }
    }
    #[allow(dead_code)]
    pub fn print_bitmasks(&self) {
        println!("all white");
        Board::print_bitmask(&self.white_all);
        println!("all black");
        Board::print_bitmask(&self.black_all);
        println!("enpasant");
        Board::print_bitmask(&self.enpasant);
        println!("castlerights");
        println!("{}",self.castle_rights);
    }
    pub fn get_int_board(&self) -> [[u8; 8]; 8] {
        let mut board: [[u8; 8]; 8] = [[12; 8]; 8];
        for i in 0u8..6 {
            for j in 0..64 {
                if self.white_pieces[i as usize] & (1u64 << j) != 0 {
                    let (x, y) = Board::getnormal(j);
                    board[y as usize][x as usize] = i;
                }
            }
            for j in 0..64 {
                if self.black_pieces[i as usize] & (1u64 << j) != 0 {
                    let (x, y) = Board::getnormal(j);
                    board[y as usize][x as usize] = 6 + i;
                }
            }
        }
        board
    }
    #[allow(dead_code)]
    pub fn print_raw_int_board(board: &[[u8; 8]; 8]) {
        for i in (0usize..8).rev() {
            for j in 0usize..8 {
                print!("{} ", board[i][j])
            }
            println!();
        }
    }

    pub fn print_int_board(board: &[[u8; 8]; 8]) {
        let mut stringboard: [String; 8] = std::array::from_fn(|_| String::new());
        for i in (0..8).rev() {
            for j in 0usize..8 {
                if board[i][j] == 12 {
                    stringboard[i].push(' ');
                    continue;
                }
                let colour = match board[i][j] {
                    x if x >= 6 => true,
                    _ => false,
                };
                let mut piecestr = match board[i][j] % 6 {
                    0 => 'p',
                    1 => 'r',
                    2 => 'n',
                    3 => 'b',
                    4 => 'q',
                    5 => 'k',
                    _ => '.',
                };
                if !colour {
                    piecestr = piecestr.to_ascii_uppercase();
                }
                stringboard[i].push(piecestr);
            }
        }
        for i in (0..8).rev() {
            println!("{}", stringboard[i]);
        }
    }
    pub fn print_board(&self){
        let int_board = self.get_int_board();
        Board::print_int_board(&int_board);
    }
    pub fn is_inside_board(x:i8, y:i8) -> bool{
        0 <= x && x < 8 && 0<= y && y < 8
    }
    pub fn is_empty(&self,x:i8, y:i8) -> bool{
        let i = Board::getpos(x,y);
        return (self.white_all & (1u64 << i) == 0) && (self.black_all & (1u64 << i) == 0);
    }
    pub fn excecute_move(&mut self, bit_move:&BitMove){
        
        for i in 0usize..6{
            self.white_pieces[i] = self.white_pieces[i]^bit_move.white_flips[i];
            self.black_pieces[i] = self.black_pieces[i]^bit_move.black_flips[i];
        }
        self.white_all = self.white_all^bit_move.white_flips[6];
        self.black_all = self.black_all^bit_move.black_flips[6];
        self.enpasant = self.enpasant^bit_move.enpasant_flip;   
        self.castle_rights = self.castle_rights^bit_move.castle_flip;
    }
    pub fn get_all_moves(&self,colour:&Colour) -> Vec<IntMove>{
        let mut ret:Vec<IntMove> = Vec::new();
        let alloop:&u64 = match colour{
            Colour::White => &self.white_all,
            Colour::Black => &self.black_all,
        };
        let piecearr:&[u64;6] = match colour{
            Colour::White => &self.white_pieces,
            Colour::Black => &self.black_pieces,
        };
        for i in 0..64{
            if alloop & (1u64 << i) == 0{
                continue;
            }
            for pi in 0usize..6{
                if piecearr[pi] & (1u64 << i) == 0{
                    continue;
                }
                let (sx,sy) = Board::getnormal(i);
                ret.extend(Piece::generate_moves(sx, sy, self, colour, &Piece::from_index(pi).unwrap()));
            }
        }
        ret
    }
    pub fn in_check(&self, colour:&Colour)->bool{
        let all_opp = self.get_all_moves(&Colour::opposite(colour));
        for opp_move in all_opp{
            let kingpos = match colour {
                Colour::White => Board::frombitpos(self.white_pieces[Piece::get_index(Piece::King)]),
                Colour::Black => Board::frombitpos(self.black_pieces[Piece::get_index(Piece::King)]),
            };
            if opp_move.ex == kingpos.0 && opp_move.ey == kingpos.1{
                return true;
            }
        }
        return false;
    }
    pub fn is_attacked(&self, colour:&Colour, x:i8, y:i8) -> bool{
        let all_opp = self.get_all_moves(&Colour::opposite(colour));
        for opp_move in all_opp{
            let kingpos = match colour {
                Colour::White => Board::frombitpos(self.white_pieces[Piece::get_index(Piece::King)]),
                Colour::Black => Board::frombitpos(self.black_pieces[Piece::get_index(Piece::King)]),
            };
            if opp_move.ex == x && opp_move.ey == y{
                return true;
            }
        }
        return false;
    }
    pub fn get_valid_moves(&mut self,colour:&Colour)->Vec<IntMove>{
        let all_raw = [self.get_all_moves(&colour),Piece::generate_castle(self, colour)].concat();
        let mut all_valid : Vec<IntMove> = Vec::new();
        for x in all_raw{
            let bit_move = BitMove::from_int_move(x, self, &colour).unwrap();
            self.excecute_move(&bit_move);
            if !self.in_check(colour){
                all_valid.push(x);
            }
            self.excecute_move(&bit_move);
        }
        return all_valid;
    }
}
