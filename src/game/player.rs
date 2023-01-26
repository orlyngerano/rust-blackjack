use super::cards;

pub struct Player {
    name: String,
    cards: Vec<cards::Card>,
    bet_on_cards: bool,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            cards: Vec::new(),
            bet_on_cards: false,
        }
    }

    pub fn add_card(&mut self, card: cards::Card) {
        self.cards.push(card);
    }

    pub fn empty_cards(&mut self) {
        self.cards.clear();
    }

    pub fn get_cards(&self) -> &Vec<cards::Card> {
        &self.cards
    }

    pub fn get_cards_points(&self) -> u8 {
        let points = self
            .cards
            .iter()
            .fold(0, |acc, x| acc + cards::get_card_value(x));

        points
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn is_bet_on_cards(&self) -> bool {
        self.bet_on_cards
    }

    pub fn set_bet_on_cards(&mut self, bet_on_cards: bool) {
        self.bet_on_cards = bet_on_cards
    }
}
