#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum GameRoundResult {
    PlayerBusted,
    DealerBusted,
    PlayerWon,
    DealerWon,
    Draw,
}

impl GameRoundResult {
    pub fn is_player_win(&self) -> bool {
        matches!(self, Self::PlayerWon | Self::DealerBusted)
    }

    pub fn is_dealer_win(&self) -> bool {
        matches!(self, Self::DealerWon | Self::PlayerBusted)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum State {
    GameStart,
    GameEnd,
    GameRoundStart,
    GameRoundEnd,
    PlayerTurn,
    DealerTurn,
}
