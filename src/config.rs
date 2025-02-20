use configparser::ini::Ini;
use std::{fs, path::PathBuf};
use std::collections::HashMap;

use crate::constants::{CONFIG_DIR, CONFIG_FILE_NAME};


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

pub fn get_lqs_directory() -> PathBuf {
  let homedir = dirs::home_dir().unwrap_or_else(|| panic!("Cannot find home directory, create home directory to continue."));
  let mut lqs_dir = homedir.clone();
  lqs_dir.push(CONFIG_DIR);
  return lqs_dir;
}

pub fn get_config_file_path() -> PathBuf {
  let mut dir = get_lqs_directory();
  dir.push(CONFIG_FILE_NAME);
  return dir;
}

pub fn create_config() {
  let lqs_dir = get_lqs_directory();
  let config_path = get_config_file_path();

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
  let config_path = get_config_file_path();

  // TODO Model this
  let map = config.load(config_path);
  return map;
}

/// Validate database connection that is passed in
pub fn validate_connection(connection: Option<String>) -> Result<String, &'static str> {
  match connection {
      Some(connection_value) => {
          let connection_name = connection_value;
          let config_loaded: Result<HashMap<String, HashMap<String, Option<String>>>, String> = load();
          if config_loaded.is_ok() && config_loaded.unwrap().get(&connection_name) != None {
              // Validate connection is the right struct
              if Connection::from_config(connection_name.to_string()).name.is_empty() {
                  return Err("Connection not found");
              }
              return Ok(connection_name.clone());
          } else {
              return Err("Connection not found");
          }
      }
      None => {
          return Err("--connection not set");
      } 
  }
}