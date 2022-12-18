use std::env;

use deerwatcher::utils::{copy, execute, watcher, config};

fn main() {
  let args: Vec<String> = env::args().collect();

  let json_path = &args
    .get(1)
    .ok_or("No json path provided")
    .unwrap();
  
  let config = config::get_config(json_path).unwrap();

  let directions = &config.list;

  let command: &str = match &config.command {
    Some(command) => command,
    None => ""
  };

  let handle_event = execute::get_event_handler(command);

  directions.iter().for_each(|direction| {
    copy::copy_recursive(
      &direction.src,
      &direction.dest,
      &direction.ignore
    ).unwrap();
  });

  watcher::watch_and_copy(&directions, &handle_event).unwrap();
}
