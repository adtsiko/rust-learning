/*
High score tracker for number guesser for example but it could be for any game
Features: 
    1 - Save scores to scores.json
    2 - Top 10 by attempts
    3 - Lookup by player name
    4 - add, list, top commands
    5 - Logic in lib

*/
use clap::{Parser, Subcommand};
use leaderboard_cli::Score;

#[derive(Parser)]
#[command(name = "leaderboard")]
#[command(about = "High score tracker")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        user: String,
        score: u32,
    },
    List,
    Top {
        #[arg(default_value_t = 2)]
        limit: u8,
    },
}

fn main() {

    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Add { user, score } => {
            let score = Score::new(user, score);
            leaderboard_cli::save_scores(score)
        },
        Commands::List => leaderboard_cli::list_scores(),
        Commands::Top { limit } => leaderboard_cli::top_scores(limit),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }

}
