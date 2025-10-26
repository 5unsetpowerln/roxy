mod enter;
mod kill;

use clap::{Parser, Subcommand};
use std::path::Path;

use crate::domain::repo::EnvSpecifier;
use crate::domain::usecase::{self, Action};

#[derive(Debug, Parser)]
struct Args {
    #[clap(subcommand)]
    sub_command: SubCommand,
}

#[derive(Debug, Subcommand)]
enum SubCommand {
    Init,
    Enter(enter::Args),
    List,
    Kill(enter::Args),
}

fn cli_subcommand_to_usecase_action(sub_command: SubCommand) -> Action {
    match sub_command {
        SubCommand::Init => Action::Init,
        SubCommand::Enter(args) => {
            if let Some(name) = args.name {
                Action::Enter(Some(EnvSpecifier::Name(name)))
            } else if let Some(path) = args.path {
                Action::Enter(Some(EnvSpecifier::Path(path)))
            } else if let Some(uuid) = args.uuid {
                Action::Enter(Some(EnvSpecifier::Uuid(uuid)))
            } else {
                Action::Enter(None)
            }
        }
        SubCommand::List => Action::List,
        SubCommand::Kill(args) => {
            if let Some(name) = args.name {
                Action::Kill(Some(EnvSpecifier::Name(name)))
            } else if let Some(path) = args.path {
                Action::Kill(Some(EnvSpecifier::Path(path)))
            } else if let Some(uuid) = args.uuid {
                Action::Kill(Some(EnvSpecifier::Uuid(uuid)))
            } else {
                Action::Kill(None)
            }
        }
    }
}

pub fn handle(current_path: &Path, database_path: &Path) {
    let args = Args::parse();

    let action = cli_subcommand_to_usecase_action(args.sub_command);

    usecase::handle(action, current_path, database_path);
}
