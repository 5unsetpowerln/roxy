use clap::{Parser, Subcommand};
use std::path::Path;
use std::process::exit;

#[derive(Debug, Parser)]
struct Args {
    #[clap(subcommand)]
    sub_command: SubCommand,
}

#[derive(Debug, Subcommand)]
enum SubCommand {
    Init,
    List,
    Enter,
    Kill,
}

pub fn handle(shared_dir_path: &Path) {
    let args = Args::parse();

    let code = match args.sub_command {
        SubCommand::Init => 0,
        SubCommand::List => 0,
        SubCommand::Enter => 0,
        SubCommand::Kill => 0,
    };

    exit(code);
}
