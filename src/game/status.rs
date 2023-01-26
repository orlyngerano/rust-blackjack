#[derive(PartialEq, Eq, Debug)]
pub enum GameRoundResult {
    PlayerBusted,
    DealerBusted,
    PlayerWon,
    DealerWon,
    Draw,
}

#[derive(PartialEq, Eq)]
pub enum State {
    GameStart,
    GameEnd,
    GameRoundStart,
    GameRoundEnd,
    PlayerTurn,
    DealerTurn,
}
