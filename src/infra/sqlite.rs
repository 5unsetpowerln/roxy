use std::path::Path;

use rusqlite::Connection;
use uuid::Uuid;

use crate::domain::repo::{EnvStore, Error};

pub struct SqliteForContainerStore {
    connection: Connection,
}

impl SqliteForContainerStore {
    pub fn new(database_path: &Path) -> Result<Self, Error> {
        // データベースに接続する (ファイルがなければ作成される)
        let connection = match Connection::open(database_path) {
            Ok(c) => c,
            Err(err) => {
                return Err(Error::Db(err));
            }
        };

        // テーブルが未作成の場合は作成する
        if let Err(err) = connection.execute(
            "CREATE TABLE IF NOT EXISTS env_records (
                        uuid          TEXT PRIMARY KEY,
                        path  TEXT NOT NULL,
                        name  TEXT NOT NULL,
                        container_id  TEXT NOT NULL
                     )",
            (),
        ) {
            return Err(Error::Db(err));
        }

        Ok(Self { connection })
    }

    // 文字列の組からEnvRecordを作成する
    fn record_from_parts(
        uuid_s: String,
        path_s: String,
        name_s: String,
        container_id_s: String,
    ) -> Result<crate::domain::repo::EnvRecord, Error> {
        let uuid = uuid::Uuid::parse_str(&uuid_s).map_err(|err| Error::Uuid(err))?;
        let spec = crate::domain::repo::EnvSpec {
            uuid,
            project_path: std::path::PathBuf::from(path_s),
            project_name: name_s,
        };
        let container_info = crate::domain::repo::ContainerInfo {
            container_id: crate::domain::repo::ContainerId::from_str(&container_id_s),
        };
        Ok(crate::domain::repo::EnvRecord {
            spec,
            container_info,
        })
    }
}

impl EnvStore for SqliteForContainerStore {
    // EnvRecordを追加する
    fn insert(&mut self, record: &crate::domain::repo::EnvRecord) -> Result<(), Error> {
        // EnvRecordの各フィールドを文字列に変換する
        let uuid = &record.spec.uuid.to_string();
        let path = &record.spec.project_path.to_string_lossy().to_string();
        let name = &record.spec.project_name;
        let container_id = &record.container_info.container_id.to_string();

        let mut stmt = self
            .connection
            .prepare(
                "INSERT INTO env_records (uuid, path, name, container_id)
                         VALUES (?1, ?2, ?3, ?4)",
            )
            .map_err(|err| Error::Db(err))?;

        stmt.execute(rusqlite::params![uuid, path, name, container_id])
            .map_err(|err| Error::Db(err))?;

        Ok(())
    }

    fn find_by_path(
        &mut self,
        path: &Path,
    ) -> Result<Vec<crate::domain::repo::EnvRecord>, crate::domain::repo::Error> {
        let path_s = path.to_string_lossy().to_string();
        let mut stmt = self
            .connection
            .prepare("SELECT uuid, path, name, container_id FROM env_records WHERE path = ?1")
            .map_err(|err| Error::Db(err))?;

        let rows = stmt
            .query_map(rusqlite::params![path_s], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                ))
            })
            .map_err(|err| Error::Db(err))?;

        let mut out = Vec::new();
        for r in rows {
            let (u, p, n, c) = r.map_err(|err| Error::Db(err))?;
            out.push(Self::record_from_parts(u, p, n, c)?);
        }
        Ok(out)
    }

    fn find_by_name(
        &mut self,
        name: String,
    ) -> Result<Vec<crate::domain::repo::EnvRecord>, crate::domain::repo::Error> {
        let mut stmt = self
            .connection
            .prepare("SELECT uuid, path, name, container_id FROM env_records WHERE name = ?1")
            .map_err(|err| Error::Db(err))?;

        let rows = stmt
            .query_map(rusqlite::params![name], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                ))
            })
            .map_err(|err| Error::Db(err))?;

        let mut out = Vec::new();
        for r in rows {
            let (u, p, n, c) = r.map_err(|err| Error::Db(err))?;
            out.push(Self::record_from_parts(u, p, n, c)?);
        }
        Ok(out)
    }

    fn find_by_uuid(
        &mut self,
        uuid: Uuid,
    ) -> Result<Vec<crate::domain::repo::EnvRecord>, crate::domain::repo::Error> {
        let uuid_s = uuid.to_string();
        let mut stmt = self
            .connection
            .prepare("SELECT uuid, path, name, container_id FROM env_records WHERE uuid = ?1")
            .map_err(|err| Error::Db(err))?;

        let rows = stmt
            .query_map(rusqlite::params![uuid_s], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                ))
            })
            .map_err(|err| Error::Db(err))?;

        let mut out = Vec::new();
        for r in rows {
            let (u, p, n, c) = r.map_err(|err| Error::Db(err))?;
            out.push(Self::record_from_parts(u, p, n, c)?);
        }
        Ok(out)
    }

    fn list(&mut self) -> Result<Vec<crate::domain::repo::EnvRecord>, crate::domain::repo::Error> {
        let mut stmt = self
            .connection
            .prepare("SELECT uuid, path, name, container_id FROM env_records")
            .map_err(|err| Error::Db(err))?;

        let rows = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                ))
            })
            .map_err(|err| Error::Db(err))?;

        let mut out = Vec::new();
        for r in rows {
            let (u, p, n, c) = r.map_err(|err| Error::Db(err))?;
            out.push(Self::record_from_parts(u, p, n, c)?);
        }
        Ok(out)
    }

    fn remove_by_path(
        &mut self,
        path: &std::path::Path,
    ) -> Result<usize, crate::domain::repo::Error> {
        let path_s = path.to_string_lossy().to_string();
        let mut stmt = self
            .connection
            .prepare("DELETE FROM env_records WHERE path = ?1")
            .map_err(|err| Error::Db(err))?;
        stmt.execute(rusqlite::params![path_s])
            .map_err(|err| Error::Db(err))
    }

    fn remove_by_name(&mut self, name: String) -> Result<usize, crate::domain::repo::Error> {
        let mut stmt = self
            .connection
            .prepare("DELETE FROM env_records WHERE name = ?1")
            .map_err(|err| Error::Db(err))?;
        stmt.execute(rusqlite::params![name])
            .map_err(|err| Error::Db(err))
    }

    fn remove_by_uuid(&mut self, uuid: uuid::Uuid) -> Result<usize, crate::domain::repo::Error> {
        let uuid_s = uuid.to_string();
        let mut stmt = self
            .connection
            .prepare("DELETE FROM env_records WHERE uuid = ?1")
            .map_err(|err| Error::Db(err))?;
        stmt.execute(rusqlite::params![uuid_s])
            .map_err(|err| Error::Db(err))
    }
}
