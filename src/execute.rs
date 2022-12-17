use std::process::{Command, Stdio};

pub fn get_event_handler <'a>(command: &'a str) -> impl Fn() + 'a {
  return move || {
    if command != "" {
      println!("Command executed!");
      let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .output()
        .expect("failed to execute process");

      println!("{}", String::from_utf8_lossy(&output.stdout));
    }
  };
}