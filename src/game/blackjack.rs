
extern crate rand;
use rand::distributions::WeightedIndex;
use rand::thread_rng;
use rand::seq::SliceRandom;

use super::cards;
use super::player::Player;
use super::status::{State, GameRoundResult};

pub struct BlackJack {
    state: State,
    dealer: Player,
    player: Player,
    card_deck: Vec<cards::Card>,
}

impl BlackJack {
    const DEALER_SOFTPOINTS: u8 = 17;
    const DEALER_NAMES: [&'static str; 4] = ["Anthony", "Bella", "Bob", "Jessy"];



    pub fn new(player: Player) -> BlackJack {
        let mut rng = thread_rng();
        let dealer_name = BlackJack::DEALER_NAMES.choose(&mut rng).unwrap_or(&&"Unknown");
     
        BlackJack {
            state: State::GameStart,
            dealer: Player::new(dealer_name.to_string()),
            player: player,
            card_deck: Vec::new()
        }
    }    

    pub fn end_game(&mut self){
        self.state = State::GameEnd;
    }

    
    fn end_game_round(&mut self){
        self.state = State::GameRoundEnd;
    }
    
    pub fn get_dealer(&self) -> &Player{
        return &self.dealer;
    }
    
    pub fn get_player(&self) -> &Player{
        return &self.player;
    }
    
    pub fn get_state(&self) -> &State{
        return &self.state;
    }

    pub fn get_winner(&self) -> GameRoundResult{
        let player_points:u8 = self.player.get_cards_points();
        let dealer_points:u8 = self.dealer.get_cards_points();

        let result = if player_points > 21 {
            GameRoundResult::PlayerBusted
        }else if dealer_points > 21 {
            GameRoundResult::DealerBusted
        }else if player_points > dealer_points{
            GameRoundResult::PlayerWon
        }else if player_points < dealer_points{
            GameRoundResult::DealerWon
        }else{
            GameRoundResult::Draw
        };

        result
    }

    pub fn player_hit_card(&mut self){
        if self.state != State::PlayerTurn {
            return;
        }

        let card: cards::Card = self.get_next_card_from_deck();
        self.player.add_card(card);
        self.state = State::DealerTurn;
        self.dealers_turn();
    }

    pub fn player_stand(&mut self){
        if self.state != State::PlayerTurn {
            return
        }
        self.player.set_bet_on_cards(true);
        self.state = State::DealerTurn;
        self.dealers_turn();
    }

    pub fn start_game(&mut self){
        self.set_card_deck();
        self.state = State::GameStart;
    }

    pub fn start_game_round(&mut self){
        self.player.empty_cards();
        self.dealer.empty_cards();
        self.player.set_bet_on_cards(false);
        self.dealer.set_bet_on_cards(false);

        self.state = State::GameRoundStart;

        let mut card:cards::Card = self.get_next_card_from_deck();
        self.player.add_card(card);
        card = self.get_next_card_from_deck();
        self.dealer.add_card(card);
        card = self.get_next_card_from_deck();
        self.player.add_card(card);
        card = self.get_next_card_from_deck();
        self.dealer.add_card(card);

        self.state = State::PlayerTurn
    }

    fn dealer_hit_card(&mut self){
        let card = self.get_next_card_from_deck();
        self.dealer.add_card(card);
    }

    fn dealers_turn(&mut self){
        if self.state != State::DealerTurn {
            return;
        }

        if self.is_dealer_want_to_hit() {
            self.dealer_hit_card();
        } else {
            self.dealer.set_bet_on_cards(true);
        }

        
        if self.dealer.get_cards_points() > 21 ||  self.player.get_cards_points() > 21{
            self.end_game_round();
        } else {
            if self.player.is_bet_on_cards() && self.dealer.is_bet_on_cards() {
                self.end_game_round();
            } else {
                if self.player.is_bet_on_cards(){
                    self.dealers_turn();
                } else{
                    self.state = State::PlayerTurn;
                }
            }
        }
        
    }

    fn get_next_card_from_deck(&mut self) -> cards::Card{
        if self.card_deck.is_empty() {
            self.set_card_deck();
        }
        return self.card_deck.remove(0);
    }

    fn is_dealer_want_to_hit(&self) -> bool{
        match self.dealer.get_cards_points() {
            points if points < BlackJack::DEALER_SOFTPOINTS => true,
            _ => false
        }
    }         

    fn set_card_deck(&mut self){
        self.card_deck.clear();
        self.card_deck.push(cards::Card::ClubsAce);
        self.card_deck.push(cards::Card::ClubsTwo);
        self.card_deck.push(cards::Card::ClubsThree);
        self.card_deck.push(cards::Card::ClubsFour);
        self.card_deck.push(cards::Card::ClubsFive);
        self.card_deck.push(cards::Card::ClubsSix);
        self.card_deck.push(cards::Card::ClubsSeven);
        self.card_deck.push(cards::Card::ClubsEight);
        self.card_deck.push(cards::Card::ClubsNine);
        self.card_deck.push(cards::Card::ClubsTen);
        self.card_deck.push(cards::Card::ClubsJack);
        self.card_deck.push(cards::Card::ClubsQueen);
        self.card_deck.push(cards::Card::ClubsKing);

        self.card_deck.push(cards::Card::DiamondsAce);
        self.card_deck.push(cards::Card::DiamondsTwo);
        self.card_deck.push(cards::Card::DiamondsThree);
        self.card_deck.push(cards::Card::DiamondsFour);
        self.card_deck.push(cards::Card::DiamondsFive);
        self.card_deck.push(cards::Card::DiamondsSix);
        self.card_deck.push(cards::Card::DiamondsSeven);
        self.card_deck.push(cards::Card::DiamondsEight);
        self.card_deck.push(cards::Card::DiamondsNine);
        self.card_deck.push(cards::Card::DiamondsTen);
        self.card_deck.push(cards::Card::DiamondsJack);
        self.card_deck.push(cards::Card::DiamondsQueen);
        self.card_deck.push(cards::Card::DiamondsKing);    

        self.card_deck.push(cards::Card::HeartsAce);
        self.card_deck.push(cards::Card::HeartsTwo);
        self.card_deck.push(cards::Card::HeartsThree);
        self.card_deck.push(cards::Card::HeartsFour);
        self.card_deck.push(cards::Card::HeartsFive);
        self.card_deck.push(cards::Card::HeartsSix);
        self.card_deck.push(cards::Card::HeartsSeven);
        self.card_deck.push(cards::Card::HeartsEight);
        self.card_deck.push(cards::Card::HeartsNine);
        self.card_deck.push(cards::Card::HeartsTen);
        self.card_deck.push(cards::Card::HeartsJack);
        self.card_deck.push(cards::Card::HeartsQueen);
        self.card_deck.push(cards::Card::HeartsKing); 

        self.card_deck.push(cards::Card::SpadesAce);
        self.card_deck.push(cards::Card::SpadesTwo);
        self.card_deck.push(cards::Card::SpadesThree);
        self.card_deck.push(cards::Card::SpadesFour);
        self.card_deck.push(cards::Card::SpadesFive);
        self.card_deck.push(cards::Card::SpadesSix);
        self.card_deck.push(cards::Card::SpadesSeven);
        self.card_deck.push(cards::Card::SpadesEight);
        self.card_deck.push(cards::Card::SpadesNine);
        self.card_deck.push(cards::Card::SpadesTen);
        self.card_deck.push(cards::Card::SpadesJack);
        self.card_deck.push(cards::Card::SpadesQueen);
        self.card_deck.push(cards::Card::SpadesKing);       

         self.shuffle_deck();   
    }

    fn shuffle_deck(&mut self){
        let mut rng = thread_rng();
        self.card_deck.shuffle(&mut rng);
    }

}
