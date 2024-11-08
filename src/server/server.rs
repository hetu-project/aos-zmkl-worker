use tokio::sync::{RwLock};
use std::sync::Arc;
use std::path::PathBuf;
use crate::config::Config;

#[derive(Debug, Clone)]
pub struct Server{
    pub config: Config,
}

#[derive(Debug, Clone)]
pub struct SharedState(pub Arc<RwLock<Server>>);

impl SharedState {
    pub async fn new(path: PathBuf) -> Self {
        let server = Server::new(path).await;
        SharedState(Arc::new(RwLock::new(server)))
    }
}

impl Server {
    pub async fn new(path: PathBuf) -> Self {
        //let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        //let manager = ConnectionManager::<PgConnection>::new(database_url);
        //let pg = Pool::builder().build(manager).expect("Failed to create pool.");

        let config = Config::load_config(path).unwrap();

        Self {
            config,
        }
    }

}
