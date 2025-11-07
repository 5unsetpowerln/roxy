use indexmap::IndexMap;
use log::debug;
use serde::{Deserialize, Serialize};
use serde_yaml::Value;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::str::FromStr;
use std::{fs, io};

use crate::domain::repo::{
    ContainerId, ContainerInfo, EnvRecord, EnvSpec, Error, Runtime, SharedResources,
};

const DOCKERFILE_NAME: &str = "dockerfile";
const COMPOSE_NAME: &str = "compose.yml";

#[derive(Debug, Serialize, Deserialize)]
struct Compose {
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,

    #[serde(default)]
    services: IndexMap<String, Service>,

    #[serde(flatten)]
    other: IndexMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Service {
    #[serde(skip_serializing_if = "Option::is_none")]
    volumes: Option<Vec<Value>>,

    #[serde(flatten)]
    other: IndexMap<String, Value>,
}

pub struct DockerForContainerRuntime {}

impl DockerForContainerRuntime {
    pub fn new() -> Self {
        Self {}
    }
}

impl Runtime for DockerForContainerRuntime {
    fn provision_and_start(
        &mut self,
        shared_resources: &SharedResources,
        env_spec: &EnvSpec,
    ) -> Result<ContainerInfo, Error> {
        // /tmp/<uuid>に設定ディレクトリを作成する
        let config_path = PathBuf::from_str(&format!("/tmp/pwnenv-{}/", env_spec.uuid)).unwrap();
        if let Err(err) = fs::remove_dir_all(&config_path) {
            if let io::ErrorKind::NotFound = err.kind() {
            } else {
                return Err(Error::Io {
                    path: Some(config_path.clone()),
                    source: io::Error::from(io::ErrorKind::Other),
                });
            }
        }

        fs::create_dir(&config_path).map_err(|err| Error::Io {
            path: Some(config_path.clone()),
            source: err,
        })?;

        // テンプレートのdockerfileとcompose.ymlを設定ディレクトリにコピーする
        // コピー先のdockerfileを作成する
        if let Err(err) = fs::File::create_new(config_path.join(DOCKERFILE_NAME)) {
            if let io::ErrorKind::AlreadyExists = err.kind() {
            } else {
                return Err(Error::Io {
                    path: Some(config_path.join(DOCKERFILE_NAME)),
                    source: io::Error::from(io::ErrorKind::Other),
                });
            }
        }
        fs::copy(
            shared_resources.dockerfile_template_absolute_path(),
            config_path.join(DOCKERFILE_NAME),
        )
        .map_err(|err| Error::Io {
            path: None,
            source: err,
        })?;

        // コピー先のcompose.tmlを作成する
        if let Err(err) = fs::File::create_new(config_path.join(COMPOSE_NAME)) {
            if let io::ErrorKind::AlreadyExists = err.kind() {
            } else {
                return Err(Error::Io {
                    path: Some(config_path.join(COMPOSE_NAME)),
                    source: io::Error::from(io::ErrorKind::Other),
                });
            }
        }
        fs::copy(
            shared_resources.compose_template_absolute_path(),
            config_path.join(COMPOSE_NAME),
        )
        .map_err(|err| Error::Io {
            path: None,
            source: err,
        })?;

        // compose.ymlの内容をシリアライズする
        // compose.ymlを開く
        let compose_file = fs::File::open(shared_resources.compose_template_absolute_path())
            .map_err(|err| Error::Io {
                path: Some(shared_resources.compose_template_absolute_path()),
                source: err,
            })?;
        let mut reader = io::BufReader::new(compose_file);
        let mut compose_contents = String::new();
        reader
            .read_to_string(&mut compose_contents)
            .map_err(|err| Error::Io {
                path: Some(shared_resources.compose_template_absolute_path()),
                source: err,
            })?;
        // シリアライズ
        let mut compose: Compose =
            serde_yaml::from_str(&compose_contents).map_err(Error::YamlSer)?;

        // compose.ymlのvolumesを編集する
        if compose.services.len() != 1 {
            // compose.ymlに一つもサービスが含まれていない、もしくは複数のサービスが含まれている場合はエラー
            return Err(Error::InvalidComposeConfig {
                reason: "services count must be exactly one".into(),
            });
        }

        let volume = format!("{}:/root/workspace:rw", env_spec.project_path.display());
        let volumes = vec![Value::String(volume)];
        compose.services[0].volumes.replace(volumes);

        // yamlにデシリアライズする
        let yaml = serde_yaml::to_string(&compose).map_err(Error::YamlDe)?;

        // 変更をcompose.ymlに保存する
        let file = fs::File::create(config_path.join(COMPOSE_NAME)).map_err(|err| Error::Io {
            path: Some(config_path.join(COMPOSE_NAME)),
            source: err,
        })?;
        let mut writer = io::BufWriter::new(file);
        writer.write_all(yaml.as_bytes()).map_err(|err| Error::Io {
            path: Some(config_path.join(COMPOSE_NAME)),
            source: err,
        })?;

        // 保存したあとにflushしないとcompose.ymlがからのままdocker compose upが実行されてしまうのでflushする
        writer.flush().map_err(|err| Error::Io {
            path: Some(config_path.join(COMPOSE_NAME)),
            source: err,
        })?;

        // docker compose up --build -dを実行する
        let status = Command::new("docker")
            .args([
                "compose",
                "-f",
                &config_path.join(COMPOSE_NAME).display().to_string(),
                "up",
                "--build",
                "-d",
            ])
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .map_err(|err| Error::Command {
                cmd: "docker compose".into(),
                status: None,
                err: err.to_string(),
            })?;

        if !status.success() {
            return Err(Error::Command {
                cmd: "docker compose".into(),
                status: None,
                err: String::new(),
            });
        }

        // 起動したコンテナのコンテナidを取得する
        let output = Command::new("docker")
            .args([
                "compose",
                "-f",
                &config_path.join(COMPOSE_NAME).display().to_string(),
                "ps",
                "-q",
            ])
            .output()
            .map_err(|err| Error::Command {
                cmd: "docker compose".into(),
                status: None,
                err: err.to_string(),
            })?;
        if !output.status.success() {
            return Err(Error::Command {
                cmd: "docker compose".into(),
                status: None,
                err: String::new(),
            });
        }

        // idの一覧を取得する
        let container_ids = String::from_utf8_lossy(&output.stdout)
            .lines()
            .filter(|l| !l.trim().is_empty())
            .map(|s| s.trim().to_string())
            .collect::<Vec<_>>();

        // 同じcompose.ymlに紐づいた環境は1つしか存在しないと仮定している
        // したがって、先頭のidだけを取得する
        if container_ids.is_empty() {
            return Err(Error::NotFound {
                what: "container id from docker compose ps",
            });
        }
        let container_id = ContainerId::from_str(&container_ids[0]);

        Ok(ContainerInfo { container_id })
    }

    fn enter(&mut self, record: &EnvRecord) {
        todo!()
    }

    fn kill(&mut self, record: &EnvRecord) -> Result<(), Error> {
        // docker killする
        let status = Command::new("docker")
            .args(["kill", &record.container_info.container_id.to_string()])
            .status()
            .map_err(|err| Error::Command {
                cmd: "docker kill".into(),
                status: None,
                err: err.to_string(),
            })?;

        if !status.success() {
            return Err(Error::Command {
                cmd: "docker kill".into(),
                status: None,
                err: String::new(),
            });
        }

        // docker rmする
        let status = Command::new("docker")
            .args(["rm", &record.container_info.container_id.to_string()])
            .status()
            .map_err(|err| Error::Command {
                cmd: "docker rm".into(),
                status: None,
                err: err.to_string(),
            })?;

        if !status.success() {
            return Err(Error::Command {
                cmd: "docker rm".into(),
                status: None,
                err: String::new(),
            });
        }

        // /tmp/<uuid>を削除する
        let config_path = PathBuf::from_iter(["/tmp", &record.spec.uuid.to_string()]);
        fs::remove_dir_all(&config_path).map_err(|err| Error::Io {
            path: None,
            source: err,
        })?;

        Ok(())
    }

    fn is_running(&mut self, record: &EnvRecord) {
        todo!()
    }
}
