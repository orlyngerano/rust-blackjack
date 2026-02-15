use blackjack::{console::BlackJackConsole, game};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the dealer
    #[arg(short, long, default_value = "Dealer")]
    dealer: String,

    /// Name of the player
    #[arg(short, long, default_value = "Player")]
    player: String,

    /// Soft points
    #[arg(short, long, default_value_t = game::blackjack::DEFAULT_SOFTPOINTS)]
    soft_points: u8,
}

fn main() {
    let args = Args::parse();
    let mut black_jack_console = BlackJackConsole::new(args.player, args.dealer, args.soft_points);
    black_jack_console.play();
}
