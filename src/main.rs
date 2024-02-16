use clap::{
    error::{Error, ErrorKind},
    ArgMatches, Args, CommandFactory, FromArgMatches, Parser, Subcommand, ValueEnum,
};
use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Settings (use 'Subcommand' to nest further commands)
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Fetch if any new games are available [Default]
    Fetch {
        /// Fetch new games available on all platforms
        #[arg(short, long)]
        all: bool,
    },

    /// Create a reminder for next games
    #[command(alias("createreminder"))]
    CreateReminder {},

    /// Add or remove platforms to search
    Platform {
        /// Add Platform [Default]
        #[arg(short, long)]
        add: bool,

        /// Remove Platform
        #[arg(short, long)]
        remove: bool,

        /// The platform
        #[arg(value_enum)]
        platform: Platform,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Platform {
    #[value(aliases(["epicgames", "epic-games"]))]
    Epic,
    #[value(aliases(["xbox", "gamepass"]))]
    GamePass,
    #[value(aliases(["playstation", "play-station", "playstation-plus"]))]
    PsPlus,
}

fn main() {
    let cli = Cli::parse();
    if cli.command.is_some() {
        match cli.command.unwrap() {
            Commands::CreateReminder {} => println!("Setting reminder on device"),
            Commands::Fetch { all } => {
                if all {
                    println!("Fetching free games from all platforms")
                } else {
                    println!("Fetching new games")
                }
            }
            Commands::Platform {
                add,
                remove,
                platform,
            } => {
                let mut default_add = false;
                if !add && !remove {
                    default_add = true
                };
                if add || default_add {
                    println!("Adding {:?} to your saved preferences", platform);
                } else {
                    println!("Removing {:?} from your saved preferences", platform);
                }
            }
        }
    } else {
        println!("Fetching new games")
    }
}
