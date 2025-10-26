mod enter;
mod init;
mod kill;
mod list;

use std::path::Path;
use std::process::exit;

use crate::infra::docker::DockerForContainerRuntime;
use crate::infra::sqlite::SqliteForContainerStore;

use self::init::InitHandler;

pub enum Action {
    Init,
    List,
    Enter,
    Kill,
}

pub fn handle(action: Action, shared_dir_path: &Path) {
    let docker = DockerForContainerRuntime::new();
    let sqlite = SqliteForContainerStore::new();

    match action {
        Action::Init => {
            let mut init_handler = InitHandler::new(docker, sqlite);
            init_handler.handle(shared_dir_path);
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
