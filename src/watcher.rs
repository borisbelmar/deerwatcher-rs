use notify::{DebouncedEvent, RecursiveMode, Watcher};
use std::{fs, path::PathBuf};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

use crate::{glob, config::Item};

enum EventType {
  Create,
  Write,
  Remove
}

pub fn watch_and_copy(directions: &Vec<Item>, on_event: &dyn Fn()) -> Result<(), std::io::Error> {
  let (tx, rx) = channel();
  let mut watcher = notify::watcher(tx, Duration::from_millis(100)).unwrap();

  for direction in directions {
    println!("Watching {}", direction.src);
    watcher.watch(Path::new(&direction.src), RecursiveMode::Recursive).unwrap();
  }

  loop {
    match rx.recv() {
      // FIXME: Its executing the process twice
      Ok(event) => {
        let event_name: EventType;
        let path_event: PathBuf;

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
          _ => {
            continue;
          },
        };

        let direction = directions.iter().find(|direction| {
          path_event.display().to_string().starts_with(&direction.src)
        }).unwrap();

        let src_path = path_event.to_str().unwrap();
        let dst_path = src_path.replace(&direction.src, &direction.dest);

        let ignored = glob::match_patterns(&direction.ignore, &path_event.as_path());

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
        }

        if !ignored {
          on_event();
        }
      },
      Err(e) => println!("Error: {:?}", e)
    }
  }
}
