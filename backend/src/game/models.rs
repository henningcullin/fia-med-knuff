pub struct Player {
    pub color: Color,
    pub user: User,
}

pub struct Piece {
    pub position: u32,
}

pub enum Color {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Pink,
}
