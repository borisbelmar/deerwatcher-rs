use notify::{DebouncedEvent, RecursiveMode, Watcher};
use std::{fs, path::PathBuf};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;

use crate::{glob, config::Item};

struct EventProps {
  ignored: bool,
  src: String,
  dest: String
}

fn get_event_props (path: &PathBuf, directions: &Vec<Item>) -> EventProps {
  let direction = directions.iter().find(|direction| {
    path.display().to_string().starts_with(&direction.src)
  }).unwrap();

  let src_path = path.to_str().unwrap();
  let dst_path = src_path.replace(&direction.src, &direction.dest);

  let ignored = glob::match_patterns(&direction.ignore, &path.as_path());

  EventProps {
    ignored,
    src: src_path.to_string(),
    dest: dst_path
  }
}

fn copy_file (event_props: &EventProps) {
  let src_path = Path::new(&event_props.src);
  let dst_path = Path::new(&event_props.dest);

  if !&event_props.ignored {
    fs::create_dir_all(dst_path.parent().unwrap()).unwrap();
    if Path::new(src_path).is_file() {
      fs::copy(src_path, dst_path).unwrap();
    }
    fs::copy(src_path, dst_path).unwrap();
  }
}

fn remove_file (event_props: &EventProps) {
  let dst_path = Path::new(&event_props.dest);

  if !&event_props.ignored {
    fs::remove_file(dst_path).unwrap();
  }
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

        match event {
          DebouncedEvent::Create(path) => {
            let event_props = get_event_props(&path, directions);
            copy_file(&event_props);
            if !&event_props.ignored {
              on_event();
            }
          },
          DebouncedEvent::Write(path) => {
            let event_props = get_event_props(&path, directions);
            copy_file(&event_props);
            if !&event_props.ignored {
              on_event();
            }
          },
          DebouncedEvent::Remove(path) => {
            let event_props = get_event_props(&path, directions);
            remove_file(&event_props);
            if !&event_props.ignored {
              on_event();
            }
          },
          _ => {
            continue;
          },
        };
      },
      Err(e) => println!("Error: {:?}", e)
    }
  }
}
