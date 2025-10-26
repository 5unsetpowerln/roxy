use std::path::Path;

use log::{error, info};

use crate::domain::repo::{EnvSpecifier, EnvStore, Runtime};

pub(crate) struct EnterHandler<R: Runtime, S: EnvStore> {
    runtime: R,
    env_store: S,
}

impl<R: Runtime, S: EnvStore> EnterHandler<R, S> {
    pub fn new(runtime: R, env_store: S) -> Self {
        Self { runtime, env_store }
    }

    pub fn handler(&mut self, current_path: &Path, env_specifier: Option<EnvSpecifier>) {
        // 環境が指定されているか確認する
        if let Some(specifier) = env_specifier {
            // 環境が指定されている場合

            // 指定子に該当する環境一覧を取得する
            let records = match self.env_store.find(specifier) {
                Ok(v) => v,
                Err(err) => {
                    error!("Failed to find environments: {err:?}");
                    return;
                }
            };

            if records.is_empty() {
                // 環境が見つからなかった場合はエラーを出して終了する
                error!("Environment not found.");
                return;
            }

            if records.len() == 1 {
                // 環境が一意に定まった場合はその環境に入る
                let record = &records[0];
                info!("Entering to {}", record.spec.project_name);
                self.runtime.enter(&record.container_info);
                return;
            }

            // 環境が一意に定まらなかった場合はエラーを出して終了する
            error!("The environment wasn't uniquely determined.");
            return;
        }

        // 環境が指定されていない場合

        // 現在存在するすべての環境の情報を取得する
        let env_records = match self.env_store.list() {
            Ok(o) => o,
            Err(err) => {
                error!("Failed to get list of environments: {err:?}");
                return;
            }
        };

        if env_records.is_empty() {
            // 一つも環境がないならその旨を伝えて終了する
            info!("No available environments.");
            return;
        }

        if env_records.len() == 1 {
            // 一つしか環境がないならその環境に入る
            let record = &env_records[0];
            info!("Entering to {}", record.spec.project_name);
            self.runtime.enter(&record.container_info);
            return;
        }

        // カレントディレクトリに紐づいた環境が存在するか確認する
        let env_record = match self.env_store.find_by_path(current_path) {
            Ok(o) => o,
            Err(err) => {
                error!("Failed to find environment: {err:?}");
                return;
            }
        };

        if let Some(env_record) = env_record {
            // 存在する場合はその環境に入る
            info!("Entering to {}", env_record.spec.project_name);
            self.runtime.enter(&env_record.container_info);
            return;
        }

        // 入るべき環境が分からない場合はエラーを出して終了する
        error!("No environment which should be entered was determined.");
    }
}
