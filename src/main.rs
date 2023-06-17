extern crate chrono;
use std::env;
use std::fs;
use std::io;
use std::path::{Path};
use std::thread;
use std::time::Duration;
use chrono::Local;
mod config;
use crate::config::{getconfig, configconstructor};


const DATE_FORMAT_STR: &'static str = "%d-%m-%Y";



fn copy_files(source_dir: &Path, destination_dir: &Path) -> io::Result<bool> {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        if args[1] == "-c" {
            configconstructor();
        }
    }
    
    let mut changed_files = false;

    if !destination_dir.exists() {
        fs::create_dir(destination_dir)?;
        println!("Missing directory {} created", destination_dir.display());
    }

    // Iterate over the entries in the source directory
    for entry in fs::read_dir(source_dir)? {
        let entry = entry?;
        let source_path = entry.path();

        if source_path.is_file() {
            let file_name = source_path.file_name().unwrap();
            let destination_path = destination_dir.join(file_name);

            // Check if the file already exists in the destination directory
            if !destination_path.exists() {
                // Copy the file to the destination directory
                fs::copy(&source_path, &destination_path)?;
                println!("Synced file to {}", destination_path.display());
                changed_files = true;
            }
        } else if source_path.is_dir() {
            let dir_name = source_path.file_name().unwrap();
            let destination_subdir = destination_dir.join(dir_name);
            match copy_files(&source_path, &destination_subdir) {
                Ok(ret) => if ret { changed_files = true; },
                Err(error) => println!("An error occurred while copying directory: {}", error),
            }
        }
    }

    // Delete files in the destination directory that are not present in the source directory
    for entry in fs::read_dir(destination_dir)? {
        let entry = entry?;
        let destination_path = entry.path();

        if destination_path.is_file() {
            let file_name = destination_path.file_name().unwrap();
            let source_path = source_dir.join(file_name);

            if !source_path.exists() {
                fs::remove_file(&destination_path)?;
                println!("Deleted file at {}", destination_path.display());
                changed_files = true;
            }
        }
    }

    Ok(changed_files)
}

fn main() {
    let conf = getconfig();
    let source_dir = Path::new(&conf.source_dir);
    
    println!("Starting to listen to changes in folder {}", source_dir.display());

    loop {
        let date = Local::now();
        let destination_dir = format!("{}-{}\\", conf.base_out_dir, date.format(DATE_FORMAT_STR));
        let destination_dir = Path::new(&destination_dir);

        match copy_files(&source_dir, &destination_dir) {
            Ok(ret) => if ret { println!("Files synced!"); },
            Err(error) => println!("An error occurred: {}", error),
        }

        thread::sleep(Duration::from_secs(5));
    }
}