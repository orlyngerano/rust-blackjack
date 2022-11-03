use std::io;

use crate::game::{
    blackjack::BlackJack,
    cards,
    player::Player,
    status::{GameRoundResult, State},
};
pub struct BlackJackConsole {
    black_jack: BlackJack,
}

impl BlackJackConsole {
    pub fn new(name: String) -> BlackJackConsole {
        BlackJackConsole {
            black_jack: BlackJack::new(Player::new(name)),
        }
    }

    fn game_round_loop(&mut self) {
        loop {
            match *self.black_jack.get_state() {
                State::GameRoundEnd => break,
                _ => {
                    self.show_player_card_message();
                    match BlackJackConsole::ask_player_play_to_hit_or_stand() {
                        true => self.black_jack.player_hit_card(),
                        false => self.black_jack.player_stand(),
                    }
                }
            }
        }
    }

    fn game_session_loop(&mut self) {
        loop {
            match *self.black_jack.get_state() {
                State::GameEnd => break,
                _ => match BlackJackConsole::ask_player_play() {
                    true => {
                        self.black_jack.start_game_round();

                        self.game_round_loop();

                        self.show_player_card_message();
                        self.show_game_round_result_message();
                    }
                    false => self.black_jack.end_game(),
                },
            }
        }
    }

    pub fn play(&mut self) {
        self.show_greetings();

        self.black_jack.start_game();

        self.game_session_loop();

        self.show_exit_message();
    }

    fn ask_player_play_to_hit_or_stand() -> bool {
        println!("Press 'h' for Hit and 's' for Stand");

        let stdin = io::stdin();
        let input = &mut String::new();

        input.clear();
        stdin.read_line(input).expect("ooops");

        let choiced = match input.trim() {
            "h" | "H" => true,
            "s" | "S" => false,
            _ => BlackJackConsole::ask_player_play_to_hit_or_stand(),
        };

        choiced
    }

    fn ask_player_play() -> bool {
        println!("Want to continue play? Press 'y' for yes and 'n' for no.");

        let stdin = io::stdin();
        let input = &mut String::new();

        input.clear();
        stdin.read_line(input).expect("oops");

        let choiced = match input.trim() {
            "n" | "N" => false,
            "y" | "Y" => true,
            _ => BlackJackConsole::ask_player_play(),
        };

        choiced
    }

    fn show_greetings(&self) {
        println!(
            r#"
---------------
Black Jack Game
---------------
"#
        );
        println!("Welcome {}", self.black_jack.get_player().get_name());
        println!(
            "I am will be dealer. My name is {}.\n",
            self.black_jack.get_dealer().get_name()
        );
    }

    fn show_exit_message(&self) {
        println!(
            "Thanks for playing. Bye {}",
            self.black_jack.get_player().get_name()
        );
    }

    fn show_player_card_message(&self) {
        let player_cards: &Vec<cards::Card> = self.black_jack.get_player().get_cards();
        let message = player_cards.iter().fold(String::new(), |mut acc, x| {
            acc.push_str(&format!(
                "{} {}\n",
                cards::get_card_name(x),
                cards::get_card_value(x)
            ));
            acc
        });
        println!("{}", message);
    }

    fn show_game_round_result_message(&self) {
        println!("-----Game Result-----");

        let winner = match self.black_jack.get_winner() {
            GameRoundResult::PlayerBusted => "Player busted. Dealer Wins".to_string(),
            GameRoundResult::DealerBusted => "Dealer busted. Player Wins".to_string(),
            GameRoundResult::PlayerWon => format!(
                "Player won. Player:{}  Dealer:{}",
                self.black_jack.get_player().get_cards_points(),
                self.black_jack.get_dealer().get_cards_points()
            ),
            GameRoundResult::DealerWon => format!(
                "Dealer won. Dealer:{}  Player:{}",
                self.black_jack.get_dealer().get_cards_points(),
                self.black_jack.get_player().get_cards_points()
            ),
            GameRoundResult::Draw => format!(
                "Draw. Dealer:{}  Player:{}",
                self.black_jack.get_dealer().get_cards_points(),
                self.black_jack.get_player().get_cards_points()
            ),
        };

        println!("{}", winner);
    }
}
