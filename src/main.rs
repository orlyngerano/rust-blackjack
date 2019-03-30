extern crate rand;

use rand::thread_rng;
use rand::seq::SliceRandom;
use std::io;

mod cards {

    #[derive(Copy, Clone)]
    pub enum Card {
        NoCard = 0x00,
        ClubsAce = 0x01,
        ClubsTwo = 0x02,
        ClubsThree = 0x03,
        ClubsFour = 0x04,
        ClubsFive = 0x05,
        ClubsSix = 0x06,
        ClubsSeven = 0x07,
        ClubsEight = 0x08,
        ClubsNine = 0x09,
        ClubsTen = 0x0A,
        ClubsJack = 0x0B,
        ClubsQueen = 0x0C,
        ClubsKing = 0x0D,
        DiamondsAce = 0x11,
        DiamondsTwo = 0x12,
        DiamondsThree = 0x13,
        DiamondsFour = 0x14,
        DiamondsFive = 0x15,
        DiamondsSix = 0x16,
        DiamondsSeven = 0x17,
        DiamondsEight = 0x18,
        DiamondsNine = 0x19,
        DiamondsTen = 0x1A,
        DiamondsJack = 0x1B,
        DiamondsQueen = 0x1C,
        DiamondsKing = 0x1D,
        HeartsAce = 0x21,
        HeartsTwo = 0x22,
        HeartsThree = 0x23,
        HeartsFour = 0x24,
        HeartsFive = 0x25,
        HeartsSix = 0x26,
        HeartsSeven = 0x27,
        HeartsEight = 0x28,
        HeartsNine = 0x29,
        HeartsTen = 0x2A,
        HeartsJack = 0x2B,
        HeartsQueen = 0x2C,
        HeartsKing = 0x2D,
        SpadesAce = 0x31,
        SpadesTwo = 0x32,
        SpadesThree = 0x33,
        SpadesFour = 0x34,
        SpadesFive = 0x35,
        SpadesSix = 0x36,
        SpadesSeven = 0x37,
        SpadesEight = 0x38,
        SpadesNine = 0x39,
        SpadesTen = 0x3A,
        SpadesJack = 0x3B,
        SpadesQueen = 0x3C,
        SpadesKing = 0x3D,
    }

    pub fn get_card_name(card: &Card) -> String {
        //let mut name: String = String::new();
        let name = match card {
            Card::NoCard => String::from("nocard"),
            Card::ClubsAce => String::from("ace of clubs"),
            Card::ClubsTwo => String::from("two of clubs"),
            Card::ClubsThree => String::from("three of clubs"),
            Card::ClubsFour => String::from("four of clubs"),
            Card::ClubsFive => String::from("five of clubs"),
            Card::ClubsSix => String::from("six of clubs"),
            Card::ClubsSeven => String::from("seven of clubs"),
            Card::ClubsEight => String::from("eight of clubs"),
            Card::ClubsNine => String::from("nine of clubs"),
            Card::ClubsTen => String::from("ten of clubs"),
            Card::ClubsJack => String::from("jack of clubs"),
            Card::ClubsQueen => String::from("queen of clubs"),
            Card::ClubsKing => String::from("king of clubs"),
            Card::DiamondsAce => String::from("ace of Diamonds"),
            Card::DiamondsTwo => String::from("two of Diamonds"),
            Card::DiamondsThree => String::from("three of Diamonds"),
            Card::DiamondsFour => String::from("four of Diamonds"),
            Card::DiamondsFive => String::from("five of Diamonds"),
            Card::DiamondsSix => String::from("six of Diamonds"),
            Card::DiamondsSeven => String::from("seven of Diamonds"),
            Card::DiamondsEight => String::from("eight of Diamonds"),
            Card::DiamondsNine => String::from("nine of Diamonds"),
            Card::DiamondsTen => String::from("ten of Diamonds"),
            Card::DiamondsJack => String::from("jack of Diamonds"),
            Card::DiamondsQueen => String::from("queen of Diamonds"),
            Card::DiamondsKing => String::from("king of Diamonds"),
            Card::HeartsAce => String::from("ace of Hearts"),
            Card::HeartsTwo => String::from("two of Hearts"),
            Card::HeartsThree => String::from("three of Hearts"),
            Card::HeartsFour => String::from("four of Hearts"),
            Card::HeartsFive => String::from("five of Hearts"),
            Card::HeartsSix => String::from("six of Hearts"),
            Card::HeartsSeven => String::from("seven of Hearts"),
            Card::HeartsEight => String::from("eight of Hearts"),
            Card::HeartsNine => String::from("nine of Hearts"),
            Card::HeartsTen => String::from("ten of Hearts"),
            Card::HeartsJack => String::from("jack of Hearts"),
            Card::HeartsQueen => String::from("queen of Hearts"),
            Card::HeartsKing => String::from("king of Hearts"),
            Card::SpadesAce => String::from("ace of Spades"),
            Card::SpadesTwo => String::from("two of Spades"),
            Card::SpadesThree => String::from("three of Spades"),
            Card::SpadesFour => String::from("four of Spades"),
            Card::SpadesFive => String::from("five of Spades"),
            Card::SpadesSix => String::from("six of Spades"),
            Card::SpadesSeven => String::from("seven of Spades"),
            Card::SpadesEight => String::from("eight of Spades"),
            Card::SpadesNine => String::from("nine of Spades"),
            Card::SpadesTen => String::from("ten of Spades"),
            Card::SpadesJack => String::from("jack of Spades"),
            Card::SpadesQueen => String::from("queen of Spades"),
            Card::SpadesKing => String::from("king of Spades"),
        };

        name
    }

    pub fn get_card_value(card: &Card) -> u8 {
       let unmask_card: u8 = 0x0f & *card as u8;
        if unmask_card > 10 {
            10
        } else {
            unmask_card
        }
    }
}

struct Player {
    name: String,
    cards: Vec<cards::Card>,
    bet_on_cards: bool,
}

impl Player {

    fn new(name: String) -> Player {
        Player {
            name: name,
            cards: Vec::new(),
            bet_on_cards: false
        }
    }

    fn add_card(&mut self, card: cards::Card) {
        self.cards.push(card);
    }

    fn empty_cards(&mut self) {
        self.cards.clear();
    }

    fn get_cards(&self) -> &Vec<cards::Card> {
        &self.cards
    }

    fn get_cards_points(&self) -> u8 {
        return self.cards.iter().fold(0, | acc, x| acc + cards::get_card_value(x));
    }

    fn get_name(&self) -> &String {
        &self.name
    }

    fn is_bet_on_cards(&self) -> bool {
        self.bet_on_cards
    }

    fn set_bet_on_cards(&mut self, bet_on_cards: bool) {
        self.bet_on_cards = bet_on_cards
    }
}

enum GameRoundResult {
    PlayerBusted,
    DealerBusted,
    PlayerWon,
    DealerWon,
    Draw,
}

#[derive(PartialEq)]
enum State {
    GameStart,
    GameEnd,
    GameRoundStart,
    GameRoundEnd,
    PlayerTurn,
    DealerTurn,
}

struct BlackJack {
    state: State,
    dealer: Player,
    player: Player,
    card_deck: Vec<cards::Card>,
}

impl BlackJack {
    const DEALER_SOFTPOINTS: u8 = 17;

    fn new(player: Player) -> BlackJack {

        BlackJack {
            state: State::GameStart,
            dealer: Player::new("Anthony".to_string()),
            player: player,
            card_deck: Vec::new()
        }
    }    

    fn end_game(&mut self){
        self.state = State::GameEnd;
    }

    
    fn end_game_round(&mut self){
        self.state = State::GameRoundEnd;
    }
    
    fn get_dealer(&self) -> &Player{
        return &self.dealer;
    }
    
    fn get_player(&self) -> &Player{
        return &self.player;
    }
    
    fn get_state(&self) -> &State{
        return &self.state;
    }

    fn get_winner(&self) -> GameRoundResult{
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

    fn player_hit_card(&mut self){
        if self.state != State::PlayerTurn {
            return;
        }

        let card: cards::Card = self.get_next_card_from_deck();
        self.player.add_card(card);
        self.state = State::DealerTurn;
        self.dealers_turn();
    }

    fn player_stand(&mut self){
        if self.state != State::PlayerTurn {
            return
        }
        self.player.set_bet_on_cards(true);
        self.state = State::DealerTurn;
        self.dealers_turn();
    }

    fn start_game(&mut self){
        self.set_card_deck();
        self.state = State::GameStart;
    }

    fn start_game_round(&mut self){
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


struct BlackJackConsole{
    black_jack: BlackJack
}

impl BlackJackConsole{
    fn new(name: String) -> BlackJackConsole{
        BlackJackConsole{
            black_jack: BlackJack::new(Player::new(name))
        }
    }

    fn play(&mut self){
        self.show_greetings();
        
        self.black_jack.start_game();

        while *self.black_jack.get_state() != State::GameEnd{
            if BlackJackConsole::ask_player_play() {
                self.black_jack.start_game_round();
                while *self.black_jack.get_state() != State::GameRoundEnd {
                    self.show_player_card_message();
                    if BlackJackConsole::ask_player_play_to_hit_or_stand(){
                        self.black_jack.player_hit_card();
                    } else {
                        self.black_jack.player_stand();
                    }
                }
                self.show_player_card_message();
                self.show_game_round_result_message();
            } else {
                self.black_jack.end_game();
            }
        }

        self.show_exit_message();
    }

    fn ask_player_play_to_hit_or_stand() -> bool{
        println!("Press 'h' for Hit and 's' for Stand");

        let stdin = io::stdin();
        let input = &mut String::new();        

        input.clear();
        stdin.read_line(input).expect("ooops");    

        let input = input.trim();    

        if input == "h" || input == "H" {
            return true;
        } else if input == "s" || input == "S" {
            return false;
        } else {
            return BlackJackConsole::ask_player_play_to_hit_or_stand();
        }        
    }

    fn ask_player_play() -> bool{
        println!("Want to continue play? Press 'y' for yes and 'n' for no.");

        let stdin = io::stdin();
        let input = &mut String::new();

        input.clear();
        stdin.read_line(input).expect("oops");

        let input = input.trim();

        if input == "n" || input == "N" {
            return false;
        } else if input == "y" || input == "Y" {
            return true;
        } else {
            return BlackJackConsole::ask_player_play();
        }
    }

    fn show_greetings(&self){
        println!("Black Jack Game");
        println!("Welcome {}", self.black_jack.get_player().get_name());
        println!("I am {}", self.black_jack.get_dealer().get_name());
    }    

    fn show_exit_message(&self){
        println!("Thanks for playing. Bye {}", self.black_jack.get_player().get_name());
    }

    fn show_player_card_message(&self){
        let player_cards: &Vec<cards::Card> = self.black_jack.get_player().get_cards();
        let message = player_cards.iter().fold(String::new(), |mut acc, x| {
            acc.push_str(&format!("{} {}\n", cards::get_card_name(x), cards::get_card_value(x)));
            acc
        });
        println!("{}", message);
    }

    fn show_game_round_result_message(&self){
        println!("-----Game Result-----");

        let winner = match self.black_jack.get_winner() {
            GameRoundResult::PlayerBusted => "Player busted. Dealer Wins".to_string(),
            GameRoundResult::DealerBusted => "Dealer busted. Player Wins".to_string(),
            GameRoundResult::PlayerWon => format!("Player won. Player:{}  Dealer:{}", self.black_jack.get_player().get_cards_points(), self.black_jack.get_dealer().get_cards_points()),
            GameRoundResult::DealerWon => format!("Dealer won. Dealer:{}  Player:{}", self.black_jack.get_dealer().get_cards_points(), self.black_jack.get_player().get_cards_points()),
            GameRoundResult::Draw => format!("Draw. Dealer:{}  Player:{}", self.black_jack.get_dealer().get_cards_points(), self.black_jack.get_player().get_cards_points()),
            _ => "".to_string()
        };

        println!("{}", winner);
    }
}

fn main() {
    let mut black_jack_console = BlackJackConsole::new("Orlyn".to_string());
    black_jack_console.play();
}