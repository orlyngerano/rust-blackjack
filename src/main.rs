use blackjack::console::BlackJackConsole;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args{
    /// Name of the dealer
    #[arg(short, long, default_value_t = String::from("Dealer"))]
    dealer: String,

    /// Name of the player
    #[arg(short, long, default_value_t =  String::from("Player"))]
    player: String,
    
    /// Softpoints
    #[arg(short, long, default_value_t = 17)]
    softpoints: u8,    
}

fn main() {
    let args = Args::parse();
    let mut black_jack_console = BlackJackConsole::new(args.player, args.dealer, args.softpoints);
    black_jack_console.play();
}