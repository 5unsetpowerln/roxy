use std::path::Path;

use clap::Parser;
use log::info;

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
            info!("Killing to {}", env_record.spec.project_name);
            self.runtime.kill(&env_record.container_info);
        }
    }
}
