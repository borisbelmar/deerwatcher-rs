use std::process::{Command, Stdio};

pub fn get_event_handler(command: &str) -> impl Fn() + '_ {
  #[allow(clippy::needless_return)]
  return move || {
    if !command.is_empty() {
      println!("Executing command: {}", command);
      Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .output()
        .expect("failed to execute process");
    }
  };
}
