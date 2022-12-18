use tempfile::NamedTempFile;
use std::io::Write;

use deerwatcher::utils::config::get_config;

#[test]
fn test_get_config () {
  let mut temp_file = NamedTempFile::new().unwrap();
  write!(temp_file, r#"{{
    "command": "echo 'Hello World!'",
    "list": [
      {{
        "src": "src",
        "dest": "dist",
        "ignore": [
          "src/**/*.js"
        ]
      }}
    ]
  }}"#).unwrap();
  
  // Get the path to the temporary file
  let file_path = temp_file.path().to_str().unwrap();

  let config = get_config(file_path).unwrap();

  assert_eq!(config.command, Some("echo 'Hello World!'".to_string()));
  assert_eq!(config.list[0].src, "src".to_string());
  assert_eq!(config.list[0].dest, "dist".to_string());
  assert_eq!(config.list[0].ignore[0], "src/**/*.js".to_string());
}

#[test]
fn test_get_config_without_command () {
  let mut temp_file = NamedTempFile::new().unwrap();
  write!(temp_file, r#"{{
    "list": [
      {{
        "src": "src",
        "dest": "dist",
        "ignore": [
          "src/**/*.js"
        ]
      }}
    ]
  }}"#).unwrap();
  
  // Get the path to the temporary file
  let file_path = temp_file.path().to_str().unwrap();

  let config = get_config(file_path).unwrap();

  assert_eq!(config.command, None);
  assert_eq!(config.list[0].src, "src".to_string());
  assert_eq!(config.list[0].dest, "dist".to_string());
  assert_eq!(config.list[0].ignore[0], "src/**/*.js".to_string());
}
