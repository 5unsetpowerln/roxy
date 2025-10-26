use crate::domain::repo::EnvStore;

pub struct SqliteForContainerStore {}

impl SqliteForContainerStore {
    pub fn new() -> Self {
        todo!()
    }
}

impl EnvStore for SqliteForContainerStore {
    fn insert(&mut self, record: &crate::domain::repo::EnvRecord) {
        todo!()
    }

    fn find_by_path(
        &self,
    ) -> Result<Option<crate::domain::repo::EnvRecord>, crate::domain::repo::Error> {
        todo!()
    }

    fn list(&self) -> Result<Vec<crate::domain::repo::EnvRecord>, crate::domain::repo::Error> {
        todo!()
    }
}
