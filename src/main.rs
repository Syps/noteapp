mod tree;

use std::env;
use std::fs::create_dir_all;
use std::io::Result;
use std::path::{Path, PathBuf};

use std::process::Command;

use chrono;
use tree::{dir_walk, Directory};

struct Config {
    root_path: String,
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let config = build_config()?;

    if args.len() > 1 && &args[1] == "list" {
        display_note_tree(&config.root_path);
        return Ok(());
    }

    let file_path = build_note_file_path(config);

    match file_path.parent() {
        Some(parent) => match make_dirs_recursive(&parent.to_path_buf()) {
            Ok(()) => println!("Ok"),
            Err(x) => eprintln!("Unable to create file paths: {x}"),
        },
        None => {}
    }
    run_and_wait_for_vim(file_path);
    Ok(())
}

fn display_note_tree(root: &str) -> Result<()> {
    let directory: Directory = dir_walk(
        &PathBuf::from(root),
        tree::is_not_hidden_name,
        tree::sort_by_name,
    )?;

    tree::print_tree(root, &directory);

    Ok(())
}

fn make_dirs_recursive(file_path: &PathBuf) -> Result<()> {
    create_dir_all(file_path)?;
    Ok(())
}

fn build_config() -> Result<Config> {
    let home_var_set = env::var("NOTEHOME");

    if home_var_set.is_err() {
        eprintln!();
        panic!("NOTEHOME must be set.");
    }

    Ok(Config {
        root_path: home_var_set.unwrap().to_string(),
    })
}

fn build_note_file_path(config: Config) -> PathBuf {
    let root_path = Path::new(&config.root_path);
    let now = chrono::offset::Local::now();
    let year = now.format("%Y").to_string();
    let month = now.format("%m").to_string();
    let day = now.format("%d.txt").to_string();

    let mut file_path = root_path.join(year);
    file_path.push(month);
    file_path.push(day);

    return file_path;
}

fn run_and_wait_for_vim(file_path: PathBuf) {
    println!("Running vim...");
    let file_path_str = file_path.as_os_str().to_str().unwrap();
    Command::new("sh")
        .arg("-c")
        .arg(format!("vim {file_path_str}"))
        .spawn()
        .expect("Error: failed to run editor")
        .wait()
        .expect("Error: Editor returned a non-zero status");
}
