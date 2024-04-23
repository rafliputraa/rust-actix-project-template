use dotenv::dotenv;

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub database_url: String,
    pub database: DatabaseConnection,
    pub server: String,
}

lazy_static! {
    pub static ref CONFIG: Config = get_config();
}

fn get_config() -> Config {
    dotenv().ok();

    match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("Configuration Error: {:#?}", error)
    }
}