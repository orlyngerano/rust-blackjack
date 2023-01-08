use blackjack::game::{status::GameRoundResult, cards::Card, player::Player, blackjack::{DEFAULT_SOFTPOINTS,BlackJack}};

#[test]
fn player_wins(){
    let player_name = String::from("Bob");
    let dealer_name = String::from("Alice");

    let mut black_jack: BlackJack = BlackJack::new(Player::new(player_name.clone()),Player::new(dealer_name.clone()), DEFAULT_SOFTPOINTS);
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
fn dealer_wins(){
    let player_name = String::from("Bob");
    let dealer_name = String::from("Alice");

    let mut black_jack: BlackJack = BlackJack::new(Player::new(player_name.clone()),Player::new(dealer_name.clone()), DEFAULT_SOFTPOINTS);
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