use std::path::{self, Path};

use uuid::Uuid;

use crate::domain::repo::EnvStore;

pub struct SqliteForContainerStore {}

impl SqliteForContainerStore {
    pub fn new(database_path: &Path) -> Self {
        todo!()
    }
}

impl EnvStore for SqliteForContainerStore {
    fn insert(&mut self, record: &crate::domain::repo::EnvRecord) {
        todo!()
    }

    fn find_by_path(
        &mut self,
        path: &Path,
    ) -> Result<Option<crate::domain::repo::EnvRecord>, crate::domain::repo::Error> {
        todo!()
    }

    fn find_by_name(
        &mut self,
        name: String,
    ) -> Result<Vec<crate::domain::repo::EnvRecord>, crate::domain::repo::Error> {
        todo!()
    }

    fn find_by_uuid(
        &mut self,
        uuid: Uuid,
    ) -> Result<Option<crate::domain::repo::EnvRecord>, crate::domain::repo::Error> {
        todo!()
    }

    fn list(&mut self) -> Result<Vec<crate::domain::repo::EnvRecord>, crate::domain::repo::Error> {
        todo!()
    }
}
