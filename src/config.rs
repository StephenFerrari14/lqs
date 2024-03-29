use configparser::ini::Ini;
use std::fs;
use std::collections::HashMap;

const CONFIG_DIR: &'static str = ".lqs";
const CONFIG_FILE_NAME: &'static str = "config";

#[derive(Debug, Clone)]
pub struct Connection {
    pub name: String,
    pub system: String,
    pub host: String,
    pub port: String,
    pub username: String,
    pub password: String,
    pub database: String,
    pub namespace: String
}

impl Connection {
  pub fn from_config(connection_name: String) -> Connection {
    let config = load().unwrap_or_else(|_| panic!("Can't load connection"));
    let connection = config.get(&connection_name).unwrap().clone();
    return Connection {
      name: connection_name.clone(),
      system: connection.get("system").unwrap().clone().unwrap(),
      host: connection.get("host").unwrap().clone().unwrap(),
      port: connection.get("port").unwrap().clone().unwrap(),
      username: connection.get("username").unwrap().clone().unwrap(),
      password: connection.get("password").unwrap().clone().unwrap(),
      database: connection.get("database").unwrap().clone().unwrap(),
      namespace: connection.get("namespace").unwrap().clone().unwrap(),
    }
  }
}

pub fn create_config() {
  let homedir = dirs::home_dir().unwrap_or_else(|| panic!("Cannot find home directory, create home directory to continue."));
  let mut lqs_dir = homedir.clone();
  lqs_dir.push(CONFIG_DIR);

  let mut config_path = homedir.clone();
  config_path.push(CONFIG_DIR);
  config_path.push(CONFIG_FILE_NAME);

  if !config_path.exists() {
    println!("Config does not exist. Creating...");

    fs::create_dir_all(lqs_dir).unwrap_or_else(|err| panic!("Error creating config, {}", err));
    let conf_path = config_path.clone();
    fs::copy("./src/config.example.ini", config_path).unwrap_or_else(|err| panic!("Error creating config, {}", err));
    println!("Created config at {}", conf_path.to_string_lossy());
  }
}

pub fn load() -> Result<HashMap<String, HashMap<String, Option<String>>>, String> {
  create_config();

  let mut config = Ini::new();
  let homedir = dirs::home_dir().unwrap_or_else(|| panic!("Cannot find home directory, create home directory to continue."));
  let mut config_path = homedir.clone();
  config_path.push(CONFIG_DIR);
  config_path.push(CONFIG_FILE_NAME);

  // TODO Model this
  let map = config.load(config_path);
  return map;
}