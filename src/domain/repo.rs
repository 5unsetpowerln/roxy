use std::path::{Path, PathBuf};

use tabled::Tabled;
use uuid::Uuid;

pub struct SharedResources {
    pub shared_dir_path: PathBuf,
    pub dockerfile_template_relative_path: PathBuf,
    pub compose_template_relative_path: PathBuf,
    pub database_relative_path: PathBuf,
}

impl SharedResources {
    pub fn dockerfile_template_absolute_path(&self) -> PathBuf {
        self.shared_dir_path
            .join(&self.dockerfile_template_relative_path)
    }

    pub fn compose_template_absolute_path(&self) -> PathBuf {
        self.shared_dir_path
            .join(&self.compose_template_relative_path)
    }

    pub fn database_absolute_path(&self) -> PathBuf {
        self.shared_dir_path.join(&self.database_relative_path)
    }
}

#[derive(Debug)]
pub enum Error {
    FailedToFind,
    DatabaseConnectionError,
    DatabaseError,
    FailedToRemove,
    FileIoError,
    SerializeError,
    DeserializeError,
    InvalidComposeConfig,
    CommandExecutionError,
    Other,
}

#[derive(Debug, Clone)]
pub struct ContainerId {
    id: String,
}

impl ContainerId {
    pub fn from_str(id: &str) -> Self {
        Self { id: id.to_string() }
    }

    pub fn to_string(&self) -> String {
        self.id.clone()
    }
}

pub enum EnvSpecifier {
    Uuid(Uuid),
    Path(PathBuf),
    Name(String),
}

#[derive(Debug, Clone)]
pub struct EnvSpec {
    pub uuid: Uuid,
    pub project_path: PathBuf,
    pub project_name: String,
}

#[derive(Debug, Clone)]
pub struct ContainerInfo {
    pub container_id: ContainerId,
}

#[derive(Debug, Clone)]
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
    fn insert(&mut self, record: &EnvRecord) -> Result<(), Error>;

    fn list(&mut self) -> Result<Vec<EnvRecord>, Error>;
    // 一つパスには一つの仮想環境しか存在しない
    fn find_by_path(&mut self, path: &Path) -> Result<Vec<EnvRecord>, Error>;

    fn find_by_name(&mut self, name: String) -> Result<Vec<EnvRecord>, Error>;
    // uuidは仮想環境を一意に定めるか、対応する仮想環境が存在しない
    fn find_by_uuid(&mut self, uuid: Uuid) -> Result<Vec<EnvRecord>, Error>;

    // pathと一致する行をすべて削除する
    fn remove_by_path(&mut self, path: &Path) -> Result<usize, Error>;
    // nameと一致する行をすべて削除する
    fn remove_by_name(&mut self, name: String) -> Result<usize, Error>;
    // uuidと一致する行をすべて削除する
    fn remove_by_uuid(&mut self, uuid: Uuid) -> Result<usize, Error>;

    fn find(&mut self, specifier: EnvSpecifier) -> Result<Vec<EnvRecord>, Error> {
        match specifier {
            EnvSpecifier::Name(name) => match self.find_by_name(name) {
                Ok(v) => Ok(v),
                Err(_err) => Err(Error::FailedToFind),
            },

            EnvSpecifier::Path(path) => match self.find_by_path(&path) {
                Ok(v) => Ok(v),
                Err(_) => Err(Error::FailedToFind),
            },

            EnvSpecifier::Uuid(uuid) => match self.find_by_uuid(uuid) {
                Ok(v) => Ok(v),
                Err(_) => Err(Error::FailedToFind),
            },
        }
    }

    fn remove(&mut self, specifier: EnvSpecifier) -> Result<usize, Error> {
        match specifier {
            EnvSpecifier::Name(name) => match self.remove_by_name(name) {
                Ok(s) => Ok(s),
                Err(_err) => Err(Error::FailedToRemove),
            },

            EnvSpecifier::Path(path) => match self.remove_by_path(&path) {
                Ok(s) => Ok(s),
                Err(_err) => Err(Error::FailedToRemove),
            },
            EnvSpecifier::Uuid(uuid) => match self.remove_by_uuid(uuid) {
                Ok(s) => Ok(s),
                Err(_err) => Err(Error::FailedToRemove),
            },
        }
    }
}

pub trait Runtime {
    fn provision_and_start(
        &mut self,
        shared_resources: &SharedResources,
        env_spec: &EnvSpec,
    ) -> Result<ContainerInfo, Error>;
    fn enter(&mut self, env_record: &EnvRecord);
    fn kill(&mut self, env_record: &EnvRecord);
    fn is_running(&mut self, env_record: &EnvRecord);
}
