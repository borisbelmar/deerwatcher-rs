use serde::{Deserialize, Serialize};
use std::fs::File;

#[derive(Serialize, Deserialize)]
pub struct Config {
  pub command: Option<String>,
  pub list: Vec<Item>
}

#[derive(Serialize, Deserialize)]
pub struct Item {
  pub src: String,
  pub dest: String,
  pub ignore: Vec<String>
}


pub fn get_config (json_path: &str) -> Result<Config, std::io::Error> {
  let file = File::open(json_path)?;
  let reader = std::io::BufReader::new(file);
  let config: Config = serde_json::from_reader(reader)?;
  Ok(config)
}
