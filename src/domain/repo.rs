use std::path::PathBuf;

use uuid::Uuid;

pub enum Error {}

pub struct ContainerId {
    id: String,
}

pub struct EnvSpec {
    pub uuid: Uuid,
    pub project_path: PathBuf,
    pub project_name: String,
}

pub struct ContainerInfo {
    container_id: ContainerId,
}

pub struct EnvRecord {
    pub spec: EnvSpec,
    pub container_info: ContainerInfo,
}

pub trait EnvStore {
    fn insert(&mut self, record: &EnvRecord);
    fn list(&self) -> Result<Vec<EnvRecord>, Error>;
    fn find_by_path(&self) -> Result<Option<EnvRecord>, Error>;
}

pub trait Runtime {
    fn provision_and_start(&self, env_spec: &EnvSpec) -> ContainerInfo;
    fn enter(&self);
    fn kill(&self);
    fn is_running(&self);
}
