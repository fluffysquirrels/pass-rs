mod args;
mod commands;
mod pgp_wrapper;
mod types;
mod utils;

use clap::Parser;
use tracing::Level;
use tracing_subscriber::{
    EnvFilter,
    filter::LevelFilter,
};

type Result<T> = std::result::Result<T, anyhow::Error>;

#[derive(clap::Parser, Clone, Debug)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand, Clone, Debug)]
enum Command {
    Find(commands::find::Args),
    Insert(commands::insert::Args),
    Show(commands::show::Args),
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .parse(std::env::var("RUST_LOG")
                       .unwrap_or("warn,pass_rs=info".to_string()))?)
        .compact()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::FULL)
        .with_file(true)
        .with_line_number(true)
        .with_target(true)
        .init();

    let args = Args::parse();

    if tracing::enabled!(Level::DEBUG) {
        tracing::debug!(args = tracing::field::debug(args.clone()), "parsed CLI args");
    }

    match args.command {
        Command::Find(cmd_args) => commands::find::main(cmd_args)?,
        Command::Insert(cmd_args) => commands::insert::main(cmd_args)?,
        Command::Show(cmd_args) => commands::show::main(cmd_args)?,
    };

    Ok(())
}
