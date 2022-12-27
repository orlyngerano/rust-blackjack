extern crate rand;

use rand::seq::SliceRandom;
use rand::thread_rng;

use super::cards;
use super::player::Player;
use super::status::{GameRoundResult, State};

pub const DEFAULT_SOFTPOINTS:u8 = 17;

pub struct BlackJack {
    state: State,
    dealer: Player,
    player: Player,
    card_deck: Vec<cards::Card>,
    soft_points: u8
}

impl BlackJack {

    pub fn new(player: Player, dealer: Player, soft_points: u8) -> BlackJack {

        BlackJack {
            state: State::GameStart,
            dealer,
            player,
            card_deck: Vec::new(),
            soft_points
        }
    }

    pub fn end_game(&mut self) {
        self.state = State::GameEnd;
    }

    fn end_game_round(&mut self) {
        self.state = State::GameRoundEnd;
    }

    pub fn get_dealer(&self) -> &Player {
        &self.dealer
    }

    pub fn get_player(&self) -> &Player {
        &self.player
    }

    pub fn get_state(&self) -> &State {
        &self.state
    }

    pub fn get_winner(&self) -> GameRoundResult {
        let player_points = self.player.get_cards_points();
        let dealer_points = self.dealer.get_cards_points();

        let winner = match (player_points, dealer_points) {
            (p, _d) if p > 21 => GameRoundResult::PlayerBusted,
            (_p, d) if d > 21 => GameRoundResult::DealerBusted,
            (p, d) if p > d => GameRoundResult::PlayerWon,
            (p, d) if d > p => GameRoundResult::DealerWon,
            _ => GameRoundResult::Draw,
        };

        winner
    }

    pub fn player_hit_card(&mut self) {
        if self.state != State::PlayerTurn {
            return;
        }

        let card: cards::Card = self.get_next_card_from_deck();
        self.player.add_card(card);
        self.state = State::DealerTurn;
        self.dealers_turn();
    }

    pub fn player_stand(&mut self) {
        if self.state != State::PlayerTurn {
            return;
        }
        self.player.set_bet_on_cards(true);
        self.state = State::DealerTurn;
        self.dealers_turn();
    }

    pub fn start_game(&mut self) {
        self.set_card_deck();
        self.state = State::GameStart;
    }

    pub fn start_game_round(&mut self) {
        for p in [&mut self.player, &mut self.dealer] {
            p.empty_cards();
            p.set_bet_on_cards(false);
        }

        self.state = State::GameRoundStart;

        for i in 1..=4 {
            let card = self.get_next_card_from_deck();
            match i % 2 {
                0 => self.player.add_card(card),
                _ => self.dealer.add_card(card),
            }
        }

        self.state = State::PlayerTurn;
    }

    fn dealer_hit_card(&mut self) {
        let card = self.get_next_card_from_deck();
        self.dealer.add_card(card);
    }

    fn dealers_turn(&mut self) {
        if self.state != State::DealerTurn {
            return;
        }

        if self.is_dealer_want_to_hit() {
            self.dealer_hit_card();
        } else {
            self.dealer.set_bet_on_cards(true);
        }

        let dealer_points = self.dealer.get_cards_points();
        let player_points = self.player.get_cards_points();
        let is_player_betting = self.player.is_bet_on_cards();
        let is_dealer_betting = self.dealer.is_bet_on_cards();

        match (dealer_points, player_points) {
            (d, p) if d > 21 || p > 21 => self.end_game_round(),
            _ => match (is_player_betting, is_dealer_betting) {
                (c, d) if c && d => self.end_game_round(),
                (c, d) if c && !d => self.dealers_turn(),
                _ => self.state = State::PlayerTurn,
            },
        }
    }

    fn get_next_card_from_deck(&mut self) -> cards::Card {
        if self.card_deck.is_empty() {
            self.set_card_deck();
        }
        self.card_deck.remove(0)
    }

    fn is_dealer_want_to_hit(&self) -> bool {
        match self.dealer.get_cards_points() {
            points if points < self.soft_points => true,
            _ => false,
        }
    }

    fn set_card_deck(&mut self) {
        self.card_deck.clear();

        for card in [
            cards::Card::ClubsAce,
            cards::Card::ClubsTwo,
            cards::Card::ClubsThree,
            cards::Card::ClubsFour,
            cards::Card::ClubsFive,
            cards::Card::ClubsSix,
            cards::Card::ClubsSeven,
            cards::Card::ClubsEight,
            cards::Card::ClubsNine,
            cards::Card::ClubsTen,
            cards::Card::ClubsJack,
            cards::Card::ClubsQueen,
            cards::Card::ClubsKing,
            cards::Card::DiamondsAce,
            cards::Card::DiamondsTwo,
            cards::Card::DiamondsThree,
            cards::Card::DiamondsFour,
            cards::Card::DiamondsFive,
            cards::Card::DiamondsSix,
            cards::Card::DiamondsSeven,
            cards::Card::DiamondsEight,
            cards::Card::DiamondsNine,
            cards::Card::DiamondsTen,
            cards::Card::DiamondsJack,
            cards::Card::DiamondsQueen,
            cards::Card::DiamondsKing,
            cards::Card::HeartsAce,
            cards::Card::HeartsTwo,
            cards::Card::HeartsThree,
            cards::Card::HeartsFour,
            cards::Card::HeartsFive,
            cards::Card::HeartsSix,
            cards::Card::HeartsSeven,
            cards::Card::HeartsEight,
            cards::Card::HeartsNine,
            cards::Card::HeartsTen,
            cards::Card::HeartsJack,
            cards::Card::HeartsQueen,
            cards::Card::HeartsKing,
            cards::Card::SpadesAce,
            cards::Card::SpadesTwo,
            cards::Card::SpadesThree,
            cards::Card::SpadesFour,
            cards::Card::SpadesFive,
            cards::Card::SpadesSix,
            cards::Card::SpadesSeven,
            cards::Card::SpadesEight,
            cards::Card::SpadesNine,
            cards::Card::SpadesTen,
            cards::Card::SpadesJack,
            cards::Card::SpadesQueen,
            cards::Card::SpadesKing,
        ] {
            self.card_deck.push(card);
        }

        self.shuffle_deck();
    }

    fn shuffle_deck(&mut self) {
        let mut rng = thread_rng();
        self.card_deck.shuffle(&mut rng);
    }
}
