use crate::{config::Connection, connector_factory, structs::{Callback, QueryContext}};

/// Run database query
pub fn run_query(connection_name: String, query: String) {
  let connection = Connection::from_config(connection_name);
  println!("Connection configured for {}...", &connection.system);
  let _ = connector_factory::submit(connection, query);
}

// Start mode to run query after query without rerunning program.
pub fn start_continuous_querying(connection_name: String) {
  let mut run = true;

  let connection = Connection::from_config(connection_name);
  println!("Connection configured for {}...", &connection.system);

  while run {
      let mut line = String::new();
      println!("Type query: (press enter to submit)");
      std::io::stdin().read_line(&mut line).unwrap();  
      if line == String::from("exit\n") {
          run = false;
      } else {
          let _ = connector_factory::submit(connection.clone(), line);
      }
  }
}

// Try to consolidate with non callback version
pub fn start_continuous_querying_callbacks(connection_name: String, query_context: Option<QueryContext>, start_callback_opt: Option<Callback>, end_callback_opt: Option<Callback>, pre_submit_callback_opt: Option<&dyn Fn(String)>, post_submit_callback_opt: Option<&dyn Fn(QueryContext, String, String)>) {
  let mut run = true;

  let context = if let Some(qcontext) = query_context {
      qcontext.clone()
  } else {
      QueryContext {
          name: "Query1".to_string()
      }
  };

  let connection = Connection::from_config(connection_name);
  println!("Connection configured for {}...", &connection.system);
  if let Some(start_callback) = start_callback_opt {
      start_callback()
  }

  while run {
      let mut line = String::new();
      println!("Type query: (press enter to submit)");
      std::io::stdin().read_line(&mut line).unwrap();  
      if line == String::from("exit\n") {
          run = false;
      } else {
          if let Some(pre_submit_callback) = pre_submit_callback_opt {
              pre_submit_callback(line.clone());
          }

          if let Ok(results) = connector_factory::submit(connection.clone(), line.clone()) {
              if let Some(post_submit_callback) = post_submit_callback_opt {
                  post_submit_callback(context.clone(), line.clone(), results.clone());
              }
          }
      }
  }

  if let Some(end_callback) = end_callback_opt {
      end_callback()
  }
}