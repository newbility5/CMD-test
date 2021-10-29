use crate::{config::Config, database::Database};
use std::env;

mod config;
mod data;
mod database;
mod repo;
mod router;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let config = match Config::parse(args) {
        Ok(config) => config,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    let (database, repo_daemon) = Database::init(&config).await;

    router::start(&config, database).await;

    repo_daemon.listen().await;
}
