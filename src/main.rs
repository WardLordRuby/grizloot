use clap::{
    error::{ContextKind, ContextValue, Error, ErrorKind},
    ArgMatches, Args, CommandFactory, FromArgMatches, Parser, Subcommand, ValueEnum,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Nested layer of optional subcommands
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

        /// Epic is [Default]
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

fn main() -> Result<(), clap::Error> {
    let cli = Cli::parse();
    if cli.command.is_some() {
        match cli.command.unwrap() {
            Commands::CreateReminder {} => println!("Setting reminder on device"),
            Commands::Fetch { all } => match all {
                true => println!("Fetching free games from all platforms"),
                false => println!("Fetching new games"),
            },
            Commands::Platform {
                add,
                remove,
                platform,
            } => match (add, remove) {
                (true, true) => {
                    clap::Error::raw(
                            ErrorKind::ArgumentConflict,
                            "[Argument Conflict] Can not use `add` and `remove` arguments at the same time. ",
                        )
                        .exit();
                }
                (false, false) => println!("Adding {:?} to your saved preferences", platform),
                (true, false) => println!("Adding {:?} to your saved preferences", platform),
                (false, true) => println!("Removing {:?} from your saved preferences", platform),
            },
        }
    } else {
        println!("Fetching new games")
    }
    Ok(())
}
