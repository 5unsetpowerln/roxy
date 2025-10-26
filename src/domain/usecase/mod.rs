mod enter;
mod init;
mod kill;
mod list;

use std::path::Path;

use log::{error, info};

use crate::infra::docker::DockerForContainerRuntime;
use crate::infra::sqlite::SqliteForContainerStore;

use self::enter::EnterHandler;
use self::init::InitHandler;
use self::kill::KillHandler;
use self::list::ListHandler;

use super::repo::{EnvRecord, EnvSpecifier, EnvStore};

pub enum Action {
    Init,
    List,
    Enter(Option<EnvSpecifier>),
    Kill(Option<EnvSpecifier>),
}

// カレントディレクトリと環境指定子から最終的にどの環境を操作するのかを返す関数
fn specify_env_to_operate<E: EnvStore>(
    env_store: &mut E,
    current_path: &Path,
    env_specifier: Option<EnvSpecifier>,
) -> Option<EnvRecord> {
    // 環境が指定されているか確認する
    if let Some(specifier) = env_specifier {
        // 環境が指定されている場合

        // 指定子に該当する環境一覧を取得する
        let records = match env_store.find(specifier) {
            Ok(v) => v,
            Err(err) => {
                error!("Failed to find environments: {err:?}");
                return None;
            }
        };

        if records.is_empty() {
            // 環境が見つからなかった場合はエラーを出して終了する
            error!("Environment not found.");
            return None;
        }

        if records.len() == 1 {
            // 環境が一意に定まった場合はその環境に入る
            return Some(records[0].clone());
        }

        // 環境が一意に定まらなかった場合はエラーを出して終了する
        error!("The environment wasn't uniquely determined.");
        return None;
    }

    // 環境が指定されていない場合

    // 現在存在するすべての環境の情報を取得する
    let env_records = match env_store.list() {
        Ok(o) => o,
        Err(err) => {
            error!("Failed to get list of environments: {err:?}");
            return None;
        }
    };

    if env_records.is_empty() {
        // 一つも環境がないならその旨を伝えて終了する
        info!("No available environments.");
        return None;
    }

    if env_records.len() == 1 {
        // 一つしか環境がないならその環境に入る
        return Some(env_records[0].clone());
    }

    // カレントディレクトリに紐づいた環境が存在するか確認する
    let env_record = match env_store.find_by_path(current_path) {
        Ok(o) => o,
        Err(err) => {
            error!("Failed to find environment: {err:?}");
            return None;
        }
    };

    if let Some(record) = env_record {
        // 存在する場合はその環境に入る
        return Some(record);
    }

    // 入るべき環境が分からない場合はエラーを出して終了する
    error!("No environment which should be entered was determined.");
    None
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
        Action::Kill(specifier) => {
            let mut kill_handler = KillHandler::new(docker, sqlite);
            kill_handler.handle(current_path, specifier);
        }
        Action::List => {
            let mut list_handler = ListHandler::new(docker, sqlite);
            list_handler.handle();
        }
    }
}
