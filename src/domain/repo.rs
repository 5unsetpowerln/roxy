use std::path::{Path, PathBuf};

use tabled::Tabled;
use uuid::Uuid;

use crate::util::option_to_vec;

#[derive(Debug)]
pub enum Error {
    FailedToFind,
}

#[derive(Clone)]
pub struct ContainerId {
    id: String,
}

pub enum EnvSpecifier {
    Uuid(Uuid),
    Path(PathBuf),
    Name(String),
}

#[derive(Clone)]
pub struct EnvSpec {
    pub uuid: Uuid,
    pub project_path: PathBuf,
    pub project_name: String,
}

#[derive(Clone)]
pub struct ContainerInfo {
    container_id: ContainerId,
}

#[derive(Clone)]
pub struct EnvRecord {
    pub spec: EnvSpec,
    pub container_info: ContainerInfo,
}

#[derive(Tabled)]
pub struct EnvRecordForList {
    pub uuid: Uuid,
    pub path: String,
    pub name: String,
}

impl EnvRecordForList {
    pub fn from_record(record: &EnvRecord) -> Self {
        Self {
            uuid: record.spec.uuid,
            name: record.spec.project_name.clone(),
            path: record.spec.project_path.display().to_string(),
        }
    }
}

pub trait EnvStore {
    fn insert(&mut self, record: &EnvRecord);

    fn list(&mut self) -> Result<Vec<EnvRecord>, Error>;
    // 一つパスには一つの仮想環境しか存在しない
    fn find_by_path(&mut self, path: &Path) -> Result<Option<EnvRecord>, Error>;

    fn find_by_name(&mut self, name: String) -> Result<Vec<EnvRecord>, Error>;
    // uuidは仮想環境を一意に定めるか、対応する仮想環境が存在しない
    fn find_by_uuid(&mut self, uuid: Uuid) -> Result<Option<EnvRecord>, Error>;

    fn find(&mut self, specifier: EnvSpecifier) -> Result<Vec<EnvRecord>, Error> {
        match specifier {
            EnvSpecifier::Name(name) => match self.find_by_name(name) {
                Ok(records) => return Ok(records),
                Err(_err) => {
                    return Err(Error::FailedToFind);
                }
            },

            EnvSpecifier::Path(path) => match self.find_by_path(&path) {
                Ok(record) => return Ok(option_to_vec(record)),
                Err(_err) => {
                    return Err(Error::FailedToFind);
                }
            },

            EnvSpecifier::Uuid(uuid) => match self.find_by_uuid(uuid) {
                Ok(record) => return Ok(option_to_vec(record)),
                Err(_err) => return Err(Error::FailedToFind),
            },
        }
    }
}

pub trait Runtime {
    fn provision_and_start(&mut self, env_spec: &EnvSpec) -> ContainerInfo;
    fn enter(&mut self, info: &ContainerInfo);
    fn kill(&mut self, info: &ContainerInfo);
    fn is_running(&mut self);
}
