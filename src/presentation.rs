// Functions for presentation

use std::{fs::{self, File, OpenOptions}, io::{self, BufRead}};
use std::io::Write;

use crate::{config::get_lqs_directory, query::start_continuous_querying_callbacks, structs::{Callback, QueryContext}};

pub fn list() {
  let presentations_dir = get_presentations_path();
  println!("{}", presentations_dir.display());

  println!("Presentations\n-------------");

  let dirs = fs::read_dir(presentations_dir).unwrap();
  for path in dirs {
      let dirname = path.unwrap();
      if dirname.metadata().unwrap().is_dir() {
          println!("{}", dirname.file_name().into_string().unwrap())
      }
  }
}

fn get_presentations_path() -> std::path::PathBuf {
    let mut lqs_dir = get_lqs_directory();
    lqs_dir.push("presentations");
    lqs_dir
}

pub fn record(connection: String, name: String) {
  println!("Recording presentation: {}", name);
  // Start continuous querying but need to add hooks to querying
  let start_callback: Callback = || {
      println!("Each query and its results will be stored as displayed and be played back with `prez play <name>`");
      println!("Starting recording...");
  };

  let end_callback: Callback = || {
      println!("Recording saved. Play with `prez play <name>`")
  };

  let post_submit_callback: &dyn Fn(QueryContext, String, String) = &|context: QueryContext, line: String, results: String| {
      // Save results to file
      let mut lqs_dir = get_presentations_path();
      lqs_dir.push(context.name.clone());

      fs::create_dir_all(lqs_dir.clone()).unwrap_or_else(|err| panic!("Error creating config, {}", err));

      lqs_dir.push("slides.txt");

      // Find better storage format
      let slide = line.replace("\n", "\\n") + "\n" + &results.replace("\n", "\\n");
      let mut file = OpenOptions::new()
          .create(true)
          .write(true)
          .append(true)
          .open(lqs_dir.clone())
          .unwrap();

      if let Err(e) = writeln!(file, "{}", slide) {
          eprintln!("Couldn't write to file: {}", e);
      } else {
          println!("Slide saved for {}", context.name);
      }
  };

  let context = QueryContext {
      name: name.clone()
  };

  // Add context to callbacks
  start_continuous_querying_callbacks(connection, Some(context), Some(start_callback), Some(end_callback), None, Some(post_submit_callback));
}

pub fn play(name: String) {
  // Does name exist
  let mut lqs_dir = get_presentations_path();
  lqs_dir.push(name.clone());

  if lqs_dir.exists() {
      // Start continuous display
      start_presentation(name.clone());
  } else {
      println!("Presentation doesn't exist. Create a new presentation with `prez record <name>`")
  }
}

pub fn start_presentation(name: String) {
  let mut presentations_dir = get_presentations_path();
  presentations_dir.push(name.clone());
  presentations_dir.push("slides.txt");

  let file = File::open(presentations_dir).unwrap();
  let reader = io::BufReader::new(file);
  let slides: io::Result<Vec<String>> = reader.lines().collect();

  let mut run = true;

  match slides {
    Ok(lines) => {
      
      println!("Presentation: {}", name);

      let mut slides_num = 0;
      let max_slides = lines.len() - 2;
      while run {
        println!("{}", lines[slides_num].replace("\\n", "\n"));
        println!("{}", lines[slides_num+1].replace("\\n", "\n"));
        println!("Slides: {}/{}, previous slide (a), next sli(d)e, (e)nd, (r)estart", (slides_num / 2) + 1, (max_slides / 2) + 1);
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();  
        println!();
        if line == String::from("e\n") { // End
            run = false;
            println!("End of presentation.")
        } else if line == String::from("a\n") { // Previous slide
            if slides_num < 2 {
              slides_num = 0
            } else {
              slides_num = slides_num - 2;
            }
        } else if line == String::from("d\n") { // Next slide
          
          if slides_num + 2 > lines.len() - 1 {
            // Then this is the last slide
            run = false;
            println!("End of presentation.")
          } else {
            slides_num = slides_num + 2;
          }
        } else if line == String::from("r\n") { // Restart
          slides_num = 0;
        }
      }
    }
    Err(e) => eprintln!("Error reading file: {}", e),
}


  while run {
      let mut line = String::new();
      println!("Type query: (press enter to submit)");
      std::io::stdin().read_line(&mut line).unwrap();  
      if line == String::from("exit\n") {
          run = false;
      } else {
          // sd
      }
  }
}