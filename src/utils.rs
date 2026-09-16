pub enum Colour {
    White,
    Black,
}
impl Colour{
    pub fn opposite(&self) -> Colour{
        match self{
            Colour::White => Colour::Black,
            Colour::Black => Colour::White,
        }
    }
}