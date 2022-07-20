use std::env;
//use std::env::{args, Args};
use std::env::args;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    // Get new Docker root name via args
    // let mut args: Args = args();
    // let docker_root: String = args.nth(1).unwrap()
    //                                      .parse::<String>()
    //                                      .unwrap();  // TODO: Fail gracefully with match
    
    // Get new Docker root name via args
    let args: Vec<String> = args().collect();
    assert_eq!(2, args.len(), "This application accepts exactly one argument");
    let docker_root: &str = &args[1];

    // Create full path to Docker root
    let docker_root_path: PathBuf = env::current_dir().ok()
                                                      .unwrap()
                                                      .join(docker_root);
    
    // Create Docker root directory
    create_dir(&docker_root_path);
    // Create new dev environment in Docker root directory
    create_dev_env(&docker_root_path);
}

fn create_dev_env(root: &PathBuf) -> () {
    // Declare files and directories to be created
    const ENV_DIRS: [&str; 2] =  ["src",
                                  "srv"];

    const ENV_FILES: [&str; 3] = ["Dockerfile",
                                  "docker-compose.yml",
                                  "README.md"];

    // Create directories
    for dir in ENV_DIRS.iter() {
        create_dir(&(root.join(dir)));
    }

    // Create files
    for file in ENV_FILES.iter() {
        create_file(&(root.join(file)));
    }
}

// Create file if it doesn't exist TODO: better solution?
fn create_file(filepath: &PathBuf) -> () {
    if !Path::new(&filepath).exists() {
        File::create(filepath).unwrap();
    }
}

// Create directory if it doesn't exist TODO: better solution?
fn create_dir(dirpath: &PathBuf) -> () {
    if !Path::new(&dirpath).is_dir() {
        fs::create_dir(&dirpath).ok();
    }
}
