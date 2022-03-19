use blackjack::console::BlackJackConsole;

fn main() {
    let player_name = std::env::args().nth(1).unwrap_or("Guest".to_string());
    let mut black_jack_console = BlackJackConsole::new(player_name);
    black_jack_console.play();
}