use std::env;
use std::path::Path;

use clap::Parser;
use log::error;
use uuid::Uuid;

use crate::domain::repo::{EnvRecord, EnvSpec, EnvStore, Runtime};
use crate::util::get_entry_name;

pub(crate) struct InitHandler<R: Runtime, S: EnvStore> {
    runtime: R,
    env_store: S,
}

impl<R: Runtime, S: EnvStore> InitHandler<R, S> {
    pub fn new(runtime: R, env_store: S) -> Self {
        Self { runtime, env_store }
    }

    pub fn handle(&mut self, project_path: &Path) {
        // EnvSpecを構築する
        let project_name = get_entry_name(project_path);

        let id = Uuid::new_v4();

        let env_spec = EnvSpec {
            uuid: id,
            project_path: project_path.to_path_buf(),
            project_name,
        };

        // 仮想環境を立ち上げる
        let container_info = self.runtime.provision_and_start(&env_spec);

        // EnvRecordを構築する
        let env_record = EnvRecord {
            container_info,
            spec: env_spec,
        };

        // EnvRecordを保存する
        self.env_store.insert(&env_record);
    }
}
