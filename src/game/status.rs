#[derive(PartialEq, Debug)]
pub enum GameRoundResult {
    PlayerBusted,
    DealerBusted,
    PlayerWon,
    DealerWon,
    Draw,
}

#[derive(PartialEq)]
pub enum State {
    GameStart,
    GameEnd,
    GameRoundStart,
    GameRoundEnd,
    PlayerTurn,
    DealerTurn,
}
