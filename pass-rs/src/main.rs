mod args;
mod commands;
mod pgp_wrapper;
mod types;
mod utils;

use clap::Parser;

type Result<T> = std::result::Result<T, anyhow::Error>;

#[derive(clap::Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand, Debug)]
enum Command {
    Show(commands::show::Args),
    Insert(commands::insert::Args),
}

fn main() -> Result<()> {
    let args = Args::parse();

    eprintln!("Args = {args:#?}");

    match args.command {
        Command::Show(cmd_args) => commands::show::main(cmd_args)?,
        Command::Insert(cmd_args) => commands::insert::main(cmd_args)?,
    };

    Ok(())
}
