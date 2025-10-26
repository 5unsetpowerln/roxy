use std::path::Path;

use log::{error, info};

use crate::domain::repo::{EnvSpecifier, EnvStore, Runtime};

use super::specify_env_to_operate;

pub(crate) struct EnterHandler<R: Runtime, S: EnvStore> {
    runtime: R,
    env_store: S,
}

impl<R: Runtime, S: EnvStore> EnterHandler<R, S> {
    pub fn new(runtime: R, env_store: S) -> Self {
        Self { runtime, env_store }
    }

    pub fn handle(&mut self, current_path: &Path, env_specifier: Option<EnvSpecifier>) {
        if let Some(env_record) =
            specify_env_to_operate(&mut self.env_store, current_path, env_specifier)
        {
            info!("Entering to {}", env_record.spec.project_name);
            self.runtime.enter(&env_record.container_info);
        }
    }
}
