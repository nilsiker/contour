#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum WorldState {
    Paused,
    Unpaused,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Prelude,
    InGame,
    GameOver,
}
