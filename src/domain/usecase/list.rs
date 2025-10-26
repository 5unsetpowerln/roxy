use log::error;
use tabled::Table;
use tabled::settings::Style;

use crate::domain::repo::{EnvRecordForList, EnvStore, Runtime};

pub(crate) struct ListHandler<R: Runtime, S: EnvStore> {
    runtime: R,
    env_store: S,
}

impl<R: Runtime, S: EnvStore> ListHandler<R, S> {
    pub fn new(runtime: R, env_store: S) -> Self {
        Self { runtime, env_store }
    }

    pub fn handle(&mut self) {
        // 現在存在するすべての環境の一覧を取得する
        let env_records = match self.env_store.list() {
            Ok(v) => v,
            Err(err) => {
                error!("Failed to get list of environments: {err:?}");
                return;
            }
        };

        // 表として出力する

        let env_records_for_list = env_records
            .iter()
            .map(EnvRecordForList::from_record)
            .collect::<Vec<EnvRecordForList>>();

        let mut table = Table::new(env_records_for_list);
        table.with(Style::blank());

        println!("{table}");
    }
}
