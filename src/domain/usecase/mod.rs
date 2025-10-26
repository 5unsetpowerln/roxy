mod enter;
mod init;
mod kill;
mod list;

use std::path::Path;
use std::process::exit;

use crate::infra::docker::DockerForContainerRuntime;
use crate::infra::sqlite::SqliteForContainerStore;

use self::enter::EnterHandler;
use self::init::InitHandler;

use super::repo::EnvSpecifier;

pub enum Action {
    Init,
    List,
    Enter(Option<EnvSpecifier>),
    Kill,
}

pub fn handle(action: Action, current_path: &Path, database_path: &Path) {
    let docker = DockerForContainerRuntime::new();
    let sqlite = SqliteForContainerStore::new(database_path);

    match action {
        Action::Init => {
            let mut init_handler = InitHandler::new(docker, sqlite);
            init_handler.handle(current_path);
        }
        Action::Enter(specifier) => {
            let mut enter_handler = EnterHandler::new(docker, sqlite);
            enter_handler.handle(current_path, specifier);
        }
        _ => {}
    }
    // let init_handler = InitHandler::new(docker, sqlite);

    // let code = match args.sub_command {
    //     SubCommand::Init(args) => init::handle(args, shared_dir_path),
    //     SubCommand::List(args) => list::handle(args),
    //     SubCommand::Enter(args) => enter::handle(args),
    //     SubCommand::Kill(args) => kill::handle(args),
    // };

    // exit(code);
}
