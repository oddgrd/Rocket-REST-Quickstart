use dotenv::dotenv;
use rocket::config::Config;
use rocket::figment::{
    map,
    value::{Map, Value},
    Figment,
};
use std::env;
use uuid::Uuid;

pub struct Configuration {
    pub secret_key: String,
    pub database_settings: DatabaseSettings,
}

impl Configuration {
    /// Generate configuration from environment
    pub fn from_env() -> Self {
        dotenv().ok();
        let database_settings = DatabaseSettings::from_env();
        let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");

        Self {
            secret_key,
            database_settings,
        }
    }

    /// Consume configuration and return it with a random database name for testing
    pub fn with_test_db(mut self) -> Self {
        self.database_settings = DatabaseSettings::from_env().with_test_db();
        self
    }

    /// Consume configuration and return a Figment, the type which
    /// Rocket expects for custom configurations.
    pub fn build(self) -> Figment {
        let db: Map<_, Value> = map! {
            "url" => self.database_settings.connection_string().into(),
            "pool_size" => 10.into()
        };
        Config::figment()
            .merge(("databases", map!["diesel_postgres_pool" => db]))
            .merge(("secret_key", &self.secret_key))
    }
}

pub struct DatabaseSettings {
    pub base_url: String,
    pub name: String,
}

impl DatabaseSettings {
    pub fn from_env() -> Self {
        let base_url = env::var("DATABASE_BASE_URL").expect("DATABASE_BASE_URL must be set");
        let name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");

        Self { base_url, name }
    }

    pub fn with_test_db(mut self) -> Self {
        self.name = Uuid::new_v4().to_string();
        self
    }

    pub fn connection_string(&self) -> String {
        format!("{}/{}", self.base_url, self.name)
    }
}
