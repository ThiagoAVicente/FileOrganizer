# NOTE: This is my first Rust project :)

# FileOrganizer

FileOrganizer is a command-line tool written in Rust for organizing files within a directory by their extensions. It also provides the functionality to restore the original structure of the directory using a log file that is created during the organization process.

## Installation and Compilation

1. Install Rust and Cargo
2. Clone this repository

### Building the Project

#### Linux or macOS:

If you are on Linux or macOS, you can use the provided script to build the project:

```bash
bash install.sh
```

#### Manually:

```bash
cargo build --release
```

The compiled binary will be located in the `target/release` directory.

## How to Run

After compiling the project, the executable can be accessed by using `./target/release/file_organizer` in the terminal.

### Organizing Files

```bash
<path_to_bin> -d <directory_path> # change the executable path as needed
```

A log file will be created in the directory that was organized. It contains all the changes made.

### Restoring Original Structure

```bash
<path_to_bin> -r <log_file_path> # change the executable path as needed
```

## Example

Before:
![Before organization](images/before.png)

Organizing:

```bash
➤ <path_to_bin> -d ~/wallpaper/
```

![After organization](images/after.png)

Restoring:

```bash
➤ <path_to_bin> -r ~/wallpaper/.file_organizer_log
```
