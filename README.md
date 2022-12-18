# Deerwatcher

Deerwatcher is a simple and lightweight tool to watch for changes in a directory and execute a command when a change is detected. It supports a config file in json format to specify the directory to watch and the command to execute. Additionally, it supports a list of files to ignore, and copy the changed files to a destination directory.

## Example configuration file

```json
{
  "command": "echo 'Hello World!'",
  "list": [
    {
      "src": "/dir/to/watch",
      "dest": "/dir/to/copy/to",
      "ignore": [
        "**/.git/**/*",
        "**/.DS_Store"
      ]
    },
    {
      "src": "/dir/to/watch2",
      "dest": "/dir/to/copy/to2",
      "ignore": [
        "**/.git/**/*",
        "**/.DS_Store",
        "**/target/**/*"
      ]
    }
  ]
}
```

## Usage

```bash
$ deerwatcher .deerwatcher.json
```
