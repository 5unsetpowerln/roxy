mod cli;
mod domain;
mod infra;
mod util;

use log::error;
use std::env;
use std::process::exit;

use self::util::{create_dir, fs_present};

fn main() {
    // ロガーの初期化
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()
        .unwrap();

    let mut shared_dir_path = env::home_dir().unwrap();
    shared_dir_path.extend([".local", "share"]);

    // 共有ディレクトリが存在していない場合はエラー
    let shared_dir_presence = match fs_present(&shared_dir_path) {
        Ok(p) => p,
        Err(err) => {
            error!("Failed to get presence of shared directory: {err}");
            exit(1);
        }
    };

    if !shared_dir_presence {
        error!(
            "Shared directory \"{}\" doesn't exists. You should run init script.",
            shared_dir_path.display()
        );
        exit(1);
    }

    cli::handle(&shared_dir_path);
}
