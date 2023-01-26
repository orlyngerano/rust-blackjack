use blackjack::game::{
    blackjack::{BlackJack, DEFAULT_SOFTPOINTS},
    cards::Card,
    player::Player,
    status::GameRoundResult,
};

#[test]
fn player_wins() {
    let player_name = String::from("Bob 1");
    let dealer_name = String::from("Alice 1");

    let mut black_jack: BlackJack = BlackJack::new(
        Player::new(player_name.clone()),
        Player::new(dealer_name.clone()),
        DEFAULT_SOFTPOINTS,
    );
    let player = black_jack.get_player();
    player.add_card(Card::ClubsTen);

    player.add_card(Card::HeartsTen);

    let dealer = black_jack.get_dealer();
    dealer.add_card(Card::DiamondsTen);
    dealer.add_card(Card::DiamondsFive);
    dealer.add_card(Card::DiamondsTwo);

    let winner = black_jack.get_winner();

    assert!(winner == GameRoundResult::PlayerWon);
}

#[test]
fn dealer_wins() {
    let player_name = String::from("Bob 2");
    let dealer_name = String::from("Alice 2");

    let mut black_jack: BlackJack = BlackJack::new(
        Player::new(player_name.clone()),
        Player::new(dealer_name.clone()),
        DEFAULT_SOFTPOINTS,
    );
    let player = black_jack.get_player();
    player.add_card(Card::DiamondsTen);
    player.add_card(Card::DiamondsFive);
    player.add_card(Card::DiamondsTwo);

    let dealer = black_jack.get_dealer();
    dealer.add_card(Card::ClubsTen);
    dealer.add_card(Card::HeartsTen);

    let winner = black_jack.get_winner();

    assert!(winner == GameRoundResult::DealerWon);
}

#[test]
fn player_wins_dealer_busted() {
    let player_name = String::from("Bob 3");
    let dealer_name = String::from("Alice 3");

    let mut black_jack: BlackJack = BlackJack::new(
        Player::new(player_name.clone()),
        Player::new(dealer_name.clone()),
        DEFAULT_SOFTPOINTS,
    );
    let player = black_jack.get_player();
    player.add_card(Card::ClubsTen);
    player.add_card(Card::HeartsAce);

    let dealer = black_jack.get_dealer();
    dealer.add_card(Card::DiamondsTen);
    dealer.add_card(Card::DiamondsQueen);
    dealer.add_card(Card::DiamondsKing);

    let winner = black_jack.get_winner();

    assert!(winner == GameRoundResult::DealerBusted);
}

#[test]
fn dealer_wins_player_busted() {
    let player_name = String::from("Bob 4");
    let dealer_name = String::from("Alice 4");

    let mut black_jack: BlackJack = BlackJack::new(
        Player::new(player_name.clone()),
        Player::new(dealer_name.clone()),
        DEFAULT_SOFTPOINTS,
    );
    let player = black_jack.get_player();
    player.add_card(Card::ClubsTen);
    player.add_card(Card::DiamondsTen);
    player.add_card(Card::DiamondsKing);

    let dealer = black_jack.get_dealer();
    dealer.add_card(Card::HeartsNine);
    dealer.add_card(Card::DiamondsQueen);

    let winner = black_jack.get_winner();

    assert!(winner == GameRoundResult::PlayerBusted);
}
