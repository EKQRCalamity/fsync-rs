extern crate chrono;
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;
use chrono::Local;
mod config;
use crate::config::{getconfig, configconstructor};
use sha2::{Sha256, Digest};
use data_encoding::HEXLOWER;
use std::fs::File;
use std::io::{BufReader, Read};


/// THANKS https://stackoverflow.com/questions/69787906/how-to-hash-a-binary-file-in-rust
fn sha256_digest(path: &PathBuf) -> io::Result<String> {
    let input = File::open(path)?;
    let mut reader = BufReader::new(input);

    let digest = {
        let mut hasher = Sha256::new();
        let mut buffer = [0; 1024];
        loop {
            let count = reader.read(&mut buffer)?;
            if count == 0 { break }
            hasher.update(&buffer[..count]);
        }
        hasher.finalize()
    };
    Ok(HEXLOWER.encode(digest.as_ref()))
}

struct ArgPassThru {
    format: String,
    format_out: String

}

impl ArgPassThru {
    pub fn new(format: String, format_out: String) -> ArgPassThru {
        return ArgPassThru { format: format, format_out: format_out };
    }

    fn setformat(&mut self, format: String) {
        self.format = format;
    }

    fn setformat_out(&mut self, format: String) {
        self.format_out = format;
    }
}

fn handleargs(args: &mut[String]) -> ArgPassThru {
    let mut pass: ArgPassThru = ArgPassThru::new("".to_string(), "".to_string());
    if args.len() > 1 {
        let mut n: usize = 0;
        for argument in args {
            if argument == "-h" {
                println!("Following arguments are passable:\n -h            Shows this message.\n -c            Re-Enter config setup.\n -format       Change the source directory date format. (Default is none)\n -format_out   Change the output date format. (Default is none)\n -dformat      Sets the format and format_out to my defaults.");
                std::process::exit(0);
            } else if argument == "-c" {
                configconstructor();
            } else if argument == "--format" {
                let format = env::args().nth(n + 1).unwrap();
                if env::args().nth(n + 1).is_none() || format.is_empty() {
                    panic!("Command line parameter after --format must be passed.");
                } else {
                    pass.setformat(format);
                }
            } else if argument == "--format_out" {
                let format = env::args().nth(n + 1).unwrap();
                if env::args().nth(n + 1).is_none() || format.is_empty() {
                    panic!("Command line parameter after --format must be passed.");
                } else {
                    pass.setformat_out(format);
                }
            } else if argument == "--dformat" {
                pass.setformat("%Y-%m-%d".to_string());
                pass.setformat_out("%d-%m-%Y".to_string());
            }
            n += 1;
        }
    }
    return pass;
}

fn copy_files(source_dir: &Path, destination_dir: &Path) -> io::Result<bool> {
    let mut changed_files = false;

    if !destination_dir.exists() {
        fs::create_dir(destination_dir)?;
        println!("Missing directory {} created", destination_dir.display());
    }
    if !source_dir.exists() {
        fs::create_dir(source_dir)?;
        println!("Missing directory {} created", source_dir.display());
    }

    // Iterate over the entries in the source directory
    for entry in fs::read_dir(source_dir)? {
        let entry = entry?;
        let source_path = entry.path();

        if source_path.is_file() {
            let file_name = source_path.file_name().unwrap();
            let destination_path = destination_dir.join(file_name);
            // Check if the file already exists in the destination directory
            if !destination_path.exists() || sha256_digest(&source_path)? != sha256_digest(&destination_path)? {
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
    let mut args: Vec<_> = env::args().collect();
    let argpasser = handleargs(args.as_mut_slice());
    let conf = getconfig();
    let bdate = Local::now();
    let bsource_dir = format!("{}{}", conf.source_dir, bdate.format(&argpasser.format));
    
    println!("Starting to listen to changes in folder {}", bsource_dir);

    loop {
        let date = Local::now();
        let source_dir = format!("{}{}", conf.source_dir, date.format(&argpasser.format));
        let source_dir = Path::new(&source_dir);
        let destination_dir = format!("{}{}\\", conf.base_out_dir, date.format(&argpasser.format_out));
        let destination_dir = Path::new(&destination_dir);

        match copy_files(&source_dir, &destination_dir) {
            Ok(ret) => if ret { println!("Files synced!"); },
            Err(error) => println!("An error occurred: {}", error),
        }

        thread::sleep(Duration::from_secs(5));
    }
}