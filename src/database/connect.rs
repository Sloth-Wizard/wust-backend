///
/// Database connection module
/// 
pub mod db {
    use std::time::Duration;
    use sea_orm::{error::*, Database, DatabaseConnection, ConnectOptions};

    use crate::database::config::db::{Conf, Load};
    
    pub async fn new() -> Result<DatabaseConnection, DbErr> {
        let mut db_conf = ConnectOptions::new(Load::url(&Conf::new()).to_owned());
        db_conf.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true);

        Ok(Database::connect(db_conf).await.unwrap())
    }
}
