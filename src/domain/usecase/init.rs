use std::path::Path;

use log::error;
use uuid::Uuid;

use crate::domain::repo::{EnvRecord, EnvSpec, EnvStore, Runtime, SharedResources};
use crate::util::get_entry_name;

pub(crate) struct InitHandler<R: Runtime, S: EnvStore> {
    runtime: R,
    env_store: S,
}

impl<R: Runtime, S: EnvStore> InitHandler<R, S> {
    pub fn new(runtime: R, env_store: S) -> Self {
        Self { runtime, env_store }
    }

    pub fn handle(&mut self, project_path: &Path, shared_resources: &SharedResources) {
        // 指定されたディレクトリに紐づいた環境が存在するか確認する
        let env_record = match self.env_store.find_by_path(project_path) {
            Ok(o) => o,
            Err(err) => {
                error!("Failed to find the environment by path: {err}");
                return;
            }
        };

        if !env_record.is_empty() {
            // 存在する場合はエラーを出して終了する
            error!(
                "Environment in {} is already running.",
                project_path.display()
            );
            return;
        }

        // EnvSpecを構築する
        let project_name = get_entry_name(project_path);

        let id = Uuid::new_v4();

        let env_spec = EnvSpec {
            uuid: id,
            project_path: project_path.to_path_buf(),
            project_name,
        };

        // 環境を立ち上げる
        let container_info = match self.runtime.init(shared_resources, &env_spec) {
            Ok(i) => i,
            Err(err) => {
                error!("Failed to start init environment: {err:?}");

                return;
            }
        };

        // EnvRecordを構築する
        let env_record = EnvRecord {
            container_info,
            spec: env_spec,
        };

        // EnvRecordを保存する
        if self.env_store.insert(&env_record).is_err() {
            error!("Failed to store environment record.");
        }

        // 環境に入る
        if let Err(err) = self.runtime.enter(&env_record) {
            error!("Failed to enter to the environment.");
        }
    }
}
