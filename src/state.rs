use crate::config::Config;

pub struct AppState {
    pub database: crate::db::Pool,
    pub config: Config,
}
