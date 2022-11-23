use common::{dotenv::dotenv, keys::DATABASE_URL_KEY, url_initializer::UrlInitializer};
use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() {
    dotenv().ok();
    std::env::set_var(
        DATABASE_URL_KEY,
        UrlInitializer::init_database_url().as_str(),
    );
    cli::run_cli(migration::Migrator).await;
}
