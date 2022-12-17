mod copy;
mod watcher;
mod glob;
mod config;
mod execute;

use std::env;

use execute::get_event_handler;

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

  let handle_event = get_event_handler(command);

  directions.iter().for_each(|direction| {
    copy::copy_recursive(
      &direction.src,
      &direction.dest,
      &direction.ignore
    ).unwrap();
  });

  watcher::watch_and_copy(&directions, &handle_event).unwrap();
}
