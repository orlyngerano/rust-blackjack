use blackjack::game::{
    blackjack::{BlackJack, DEFAULT_SOFTPOINTS},
    player::Player,
};

#[test]
fn names() {
    let player_name = String::from("Bob");
    let dealer_name = String::from("Alice");

    let mut black_jack = BlackJack::new(
        Player::new(player_name.clone()),
        Player::new(dealer_name.clone()),
        DEFAULT_SOFTPOINTS,
    );

    let game_player_name = black_jack.get_player().get_name();
    assert!(
        game_player_name.eq(&player_name),
        "game_player_name={} player_name={}",
        game_player_name,
        player_name
    );

    let game_dealer_name = black_jack.get_dealer().get_name();
    assert!(
        game_dealer_name.eq(&dealer_name),
        "game_dealer_name={} dealer_name={}",
        game_dealer_name,
        dealer_name
    );
}
