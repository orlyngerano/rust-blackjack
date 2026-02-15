use colored::Colorize;

#[derive(Copy, Clone)]
pub enum Card {
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

fn get_suit(card: Card) -> u8 {
    (card as u8) >> 4
}

fn get_rank(card: Card) -> u8 {
    (card as u8) & 0x0F
}

pub fn get_card_name(card: &Card) -> String {
    let rank_name = match get_rank(*card) {
        1 => "ace",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "jack",
        12 => "queen",
        13 => "king",
        _ => "unknown",
    };

    let suit_name = match get_suit(*card) {
        0 => "Clubs",
        1 => "Diamonds",
        2 => "Hearts",
        3 => "Spades",
        _ => "unknown",
    };

    format!("{} of {}", rank_name, suit_name)
}

pub fn get_card_value(card: &Card) -> u8 {
    let rank = get_rank(*card);
    match rank {
        1 => 11,
        11..=13 => 10,
        n => n,
    }
}

pub fn get_card_display(card: &Card) -> Vec<String> {
    let suit = match get_suit(*card) {
        0 => "\u{2663}".to_string(),
        1 => format!("{}", "\u{2666}".red()),
        2 => format!("{}", "\u{2665}".red()),
        3 => "\u{2660}".to_string(),
        _ => "".to_string(),
    };

    let rank = match get_rank(*card) {
        1 => "A".to_string(),
        11 => "J".to_string(),
        12 => "Q".to_string(),
        13 => "K".to_string(),
        n => n.to_string(),
    };

    vec![
        "┌─────────┐".to_string(),
        format!("│{:<9}│", rank),
        "│         │".to_string(),
        format!("│    {}    │", suit),
        "│         │".to_string(),
        format!("│{:>9}│", rank),
        "└─────────┘".to_string(),
    ]
}

pub fn print_cards(cards: &[Card]) {
    if cards.is_empty() {
        return;
    }

    let ascii_cards: Vec<Vec<String>> = cards.iter().map(|c| get_card_display(c)).collect();

    for row in 0..ascii_cards[0].len() {
        for card in &ascii_cards {
            print!("{}  ", card[row]);
        }
        println!();
    }
}
