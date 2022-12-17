mod copy;
mod watcher;
mod glob;
mod config;

use std::{process::{Command, Stdio}, env};

use crate::config::get_config;



fn main() {
  let args: Vec<String> = env::args().collect();

  let json_path = &args
    .get(1)
    .ok_or("No json path provided")
    .unwrap();
  
  let config = get_config(json_path).unwrap();
  
  let directions = &config.list;
  let command = config.command.as_str();

  let handle_event = || {
    println!("Event handled");
    Command::new("sh")
      .arg("-c")
      .arg(command)
      .stdout(Stdio::piped())
      .output()
      .expect("failed to execute process");
  };

  directions.iter().for_each(|direction| {
    copy::copy_recursive(
      &direction.src,
      &direction.dest,
      &direction.ignore
    ).unwrap();
  });

  watcher::watch_and_copy(&directions, &handle_event).unwrap();
}
