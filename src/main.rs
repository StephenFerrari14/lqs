use clap::{Parser, Subcommand, Args};
use config::validate_connection;
use query::{run_query, start_continuous_querying};

mod constants;
mod lqs;
mod config;
mod connector_factory;
mod display_row;
mod presentation;
mod structs;
mod query;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Connection from config file
    #[arg(short, long)]
    connection: Option<String>,

    /// Query to run against connection
    #[arg(short, long)]
    query: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Args)]
struct PrezSubCommands {
    #[command(subcommand)]
    commands: Option<PrezCommands>
}

#[derive(Args)]
struct PrezCommandArgs {
    name: String,

    /// Connection from config file
    #[arg(short, long)]
    connection: Option<String>,
}

#[derive(Args)]
struct PrezPlayArgs {
    name: String,
}

#[derive(Subcommand)]
enum PrezCommands {
    /// Get list of presentations
    List,
    /// Record presentation
    Record(PrezCommandArgs),
    /// Play presentation
    Play(PrezPlayArgs)
}


#[derive(Subcommand)]
enum Commands {
    /// Initializes lqs cli
    Init,
    /// Presentation Mode
    Prez(PrezSubCommands)
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Init) => {
            config::create_config();
        }
        Some(Commands::Prez(subcommand)) => {
            // Refactor command functions
            match &subcommand.commands {
                Some(PrezCommands::List) => {
                    presentation::list();
                },
                Some(PrezCommands::Record(args)) => {
                    match validate_connection(args.connection.clone()) {
                        Ok(connection) => {
                            presentation::record(connection, args.name.clone());
                        },
                        Err(e) => {
                            println!("Error: {}. See --help", e);
                            return;
                        }
                    }
                },
                Some(PrezCommands::Play(args)) => {
                    presentation::play(args.name.clone());
                },
                None => todo!(),
            }
        }
        None => {
            match validate_connection(cli.connection) {
                Ok(connection) => {
                    match cli.query {
                        Some(input_query) => {
                            run_query(connection, input_query);
                        }
                        None => {
                            start_continuous_querying(connection);
                        }
                    }
                },
                Err(e) => {
                    panic!("Error: {}", e);
                }
            }
        }
    }
}