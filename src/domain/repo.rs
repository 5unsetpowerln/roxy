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

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("not found: {what}")]
    NotFound { what: &'static str },

    #[error("database connection failed: {path:?}: {source}")]
    DbConn {
        path: std::path::PathBuf,
        #[source]
        source: rusqlite::Error,
    },

    #[error("database error: {0}")]
    Db(#[from] rusqlite::Error),

    #[error("I/O error {path:?}: {source}")]
    Io {
        path: Option<std::path::PathBuf>,
        #[source]
        source: std::io::Error,
    },

    #[error("YAML serialize error: {0}")]
    YamlSer(#[source] serde_yaml::Error),

    #[error("YAML deserialize error: {0}")]
    YamlDe(#[source] serde_yaml::Error),

    #[error("invalid compose configuration: {reason}")]
    InvalidComposeConfig { reason: String },

    #[error("failed to run command: {cmd} (status: {status:?}){err}")]
    Command {
        cmd: String,
        status: Option<i32>,
        err: String,
    },

    #[error("remove failed: {what}")]
    RemoveFailed { what: String },

    #[error("template not found: {path:?}")]
    TemplateNotFound { path: std::path::PathBuf },

    #[error("invalid path: {path:?} ({msg})")]
    InvalidPath {
        path: std::path::PathBuf,
        msg: String,
    },

    #[error("environment conflict: name={name}, path={path:?}, existing={existing:?}")]
    EnvConflict {
        name: String,
        path: std::path::PathBuf,
        existing: Option<uuid::Uuid>,
    },

    #[error("uuid error: {0}")]
    Uuid(uuid::Error),
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
            EnvSpecifier::Name(name) => self.find_by_name(name),
            EnvSpecifier::Path(path) => self.find_by_path(&path),
            EnvSpecifier::Uuid(uuid) => self.find_by_uuid(uuid),
        }
    }

    fn remove(&mut self, specifier: EnvSpecifier) -> Result<usize, Error> {
        match specifier {
            EnvSpecifier::Name(name) => self.remove_by_name(name),
            EnvSpecifier::Path(path) => self.remove_by_path(&path),
            EnvSpecifier::Uuid(uuid) => self.remove_by_uuid(uuid),
        }
    }
}

pub trait Runtime {
    fn init(
        &mut self,
        shared_resources: &SharedResources,
        env_spec: &EnvSpec,
    ) -> Result<ContainerInfo, Error>;
    fn enter(&mut self, env_record: &EnvRecord) -> Result<(), Error>;
    fn kill(&mut self, env_record: &EnvRecord) -> Result<(), Error>;
}
