use std::io::{Error, ErrorKind};

use crate::{lqs, config::Connection};
mod postgres_connector;
mod surrealdb_connector;



pub fn submit(connection: Connection, query: String) -> Result<String, Error> {
  let parsed_query = lqs::parse(query);
  let system = connection.system.clone();

  match system.as_str() {
    "postgres" => {
      match postgres_connector::query(connection, parsed_query) {
        Ok(result) => {
          Ok(result)
        },
        Err(error) => {
          return Err(Error::new(ErrorKind::Other, error.to_string()))
        }
      }
    },
    "surrealdb" => {
      surrealdb_connector::query(connection, parsed_query);
      Ok("Return results NYI".to_string())
    },
    _ => Err(Error::new(ErrorKind::Other, "No system configured"))
  }
}
