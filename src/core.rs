#[derive(PartialEq, Eq)]
pub enum Orientation {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum PlayerNumber {
    PlayerOne,
    PlayerTwo
}
