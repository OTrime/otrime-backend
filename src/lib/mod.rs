use database::Database;
pub mod se;

pub mod database;
pub mod errors;
pub mod http;
pub trait IntoInner {
    type Output;
    fn into_inner(self) -> Self::Output;
}

#[derive(Debug, Clone)]
pub struct Config {
    pub db_url: String,
}

pub struct State {
    pub database: Database,
}
