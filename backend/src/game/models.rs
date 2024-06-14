use crate::auth::models::User;

pub struct Piece {
    pub position: u32,
}

pub struct Pieces {
    pub piece1: Piece,
    pub piece2: Piece,
    pub piece3: Piece,
    pub piece4: Piece,
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
pub struct Player {
    pub color: Color,
    pub user: User,
    pub pieces: Pieces,
}

pub struct Players {
    pub player1: Player,
    pub player2: Player,
    pub player3: Player,
    pub player4: Player,
}

pub struct Board {
    pub players: Players,
}
