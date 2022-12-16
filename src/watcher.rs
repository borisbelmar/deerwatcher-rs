use notify::{DebouncedEvent, RecursiveMode, Watcher};
use std::{fs, path::PathBuf};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

use crate::{glob, Direction};

enum EventType {
  Create,
  Write,
  Remove,
  Nothing
}

pub fn watch_and_copy(directions: &[Direction], ignore_list: &[&str], on_event: fn()) -> Result<(), std::io::Error> {
  let (tx, rx) = channel();
  let mut watcher = notify::watcher(tx, Duration::from_millis(100)).unwrap();

  for direction in directions {
    println!("Watching {}", direction.src_dir);
    watcher.watch(Path::new(&direction.src_dir), RecursiveMode::Recursive).unwrap();
  }

  loop {
    match rx.recv() {
      // FIXME: Its executing the process twice
      Ok(event) => {
        let mut event_name: EventType = EventType::Nothing;

        let mut path_event: PathBuf = PathBuf::new();

        match event {
          DebouncedEvent::Create(path) => {
            path_event = path;
            event_name = EventType::Create;
          },
          DebouncedEvent::Write(path) => {
            path_event = path;
            event_name = EventType::Write;
          },
          DebouncedEvent::Remove(path) => {
            path_event = path;
            event_name = EventType::Remove;
          },
          _ => {},
        };

        let direction: &Direction = directions.iter().find(|direction| {
          path_event.display().to_string().starts_with(&direction.src_dir)
        }).unwrap();

        let src_path = path_event.to_str().unwrap();
        let dst_path = src_path.replace(&direction.src_dir, &direction.dst_dir);

        let ignored = glob::match_patterns(ignore_list, &path_event.as_path());

        match event_name {
          EventType::Create => {
            if !ignored {
              fs::create_dir_all(Path::new(dst_path.as_str()).parent().unwrap()).unwrap();
              if Path::new(src_path).is_file() {
                fs::copy(src_path, dst_path).unwrap();
              }
            }
          }
          EventType::Write => {
            if !ignored {
              fs::create_dir_all(Path::new(dst_path.as_str()).parent().unwrap()).unwrap();
              fs::copy(src_path, dst_path).unwrap();
            }
          }
          EventType::Remove => {
            if !ignored {
              std::fs::remove_dir_all(dst_path).unwrap();
            }
          }
          EventType::Nothing => {}
        }

        if !ignored {
          on_event();
        }
      },
      Err(e) => println!("Error: {:?}", e)
    }
  }
}
