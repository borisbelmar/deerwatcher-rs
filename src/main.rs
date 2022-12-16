mod copy;
mod watcher;
mod glob;

pub struct Direction {
  src_dir: String,
  dst_dir: String,
}

fn main() {
  let directions = [
    Direction {
      src_dir: "/Users/borisbelmar/Proyectos/Deerwatcher/test1".to_string(),
      dst_dir: "/Users/borisbelmar/Proyectos/Deerwatcher/test2".to_string(),
    }
  ];

  let ignored_list = [".git", "**/target/**/*", "**/src/**/*"];

  fn handle_event() {
    println!("Event handled");
  }

  for direction in &directions {
    copy::copy_recursive(&direction.src_dir, &direction.dst_dir, &ignored_list).unwrap();
  }

  watcher::watch_and_copy(&directions, &ignored_list, handle_event).unwrap();
}
