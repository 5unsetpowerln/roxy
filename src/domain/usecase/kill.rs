use std::path::Path;

use clap::Parser;
use log::{error, info};

use crate::domain::repo::{EnvSpecifier, EnvStore, Runtime};

use super::specify_env_to_operate;

pub(crate) struct KillHandler<R: Runtime, S: EnvStore> {
    runtime: R,
    env_store: S,
}

impl<R: Runtime, S: EnvStore> KillHandler<R, S> {
    pub fn new(runtime: R, env_store: S) -> Self {
        Self { runtime, env_store }
    }

    pub fn handle(&mut self, current_path: &Path, env_specifier: Option<EnvSpecifier>) {
        if let Some(env_record) =
            specify_env_to_operate(&mut self.env_store, current_path, env_specifier)
        {
            let records = match self.env_store.find_by_uuid(env_record.spec.uuid) {
                Ok(o) => o,
                Err(err) => {
                    error!("Failed to find environment record: {err:?}");
                    return;
                }
            };

            // specify_env_to_operateで取得した時点でこのUuidに紐づいた環境が存在していること仮定されている
            assert!(!records.is_empty());

            // このUuidに紐づいた環境が複数存在している場合はエラー
            if records.len() != 1 {
                error!("Multiple environments with same uuid mustn't exist.");
                return;
            }

            info!("Killing {}", env_record.spec.project_name);

            // 環境を終了する
            if let Err(err) = self.runtime.kill(&env_record) {
                error!("Failed to kill the environment: {err}");
            }

            // 環境の情報を破棄する
            if let Err(err) = self.env_store.remove_by_uuid(env_record.spec.uuid) {
                error!("Failed to remove the environment record: {err}");
            }
        }
    }
}
