use dotenv;

#[derive(Debug)]
pub struct Conf {
    db_type: String,
    db_user: String,
    db_pwd: String,
    db_port: String,
    db: String,
    db_service_name: String
}

pub trait Load {
    fn new() -> Conf;
    fn url(&self) -> String;
}

impl Load for Conf {
    fn new() -> Conf {
        Conf {
            db_type: dotenv::var("DB_TYPE").unwrap(),
            db_user: dotenv::var("DB_USER").unwrap(),
            db_pwd: dotenv::var("DB_PWD").unwrap(),
            db_port: dotenv::var("DB_PORT").unwrap(),
            db: dotenv::var("DB").unwrap(),
            db_service_name: dotenv::var("DB_SERVICE_NAME").unwrap()
        }
    }

    fn url(&self) -> String {
        format!(
            "{}://{}:{}@{}:{}/{}",
            self.db_type,
            self.db_user,
            self.db_pwd,
            self.db_service_name,
            self.db_port,
            self.db
        )
    }
}
