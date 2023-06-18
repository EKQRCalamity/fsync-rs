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
#[derive(Clone)]
struct Console {
    quiet: bool,
    timestamp: bool,
    timestamp_format: String,
}

impl Console {
    pub fn new(quiet: bool, timestamp: bool, timestamp_format: String) -> Console {
        return Console { quiet: quiet, timestamp: timestamp, timestamp_format: timestamp_format };
    }

    fn print(&self, mut outstr: String) {
        if !self.quiet {
            if self.timestamp {
                let date = Local::now();
                outstr = format!("[{}] {}", date.format(self.timestamp_format.as_str()), outstr);
            }
            println!("{}", outstr);
        }
    }

    fn setquiet(&mut self, quiet: bool) {
        self.quiet = quiet;
    }
}

struct ArgPassThru {
    format: String,
    format_out: String,
    console: Console,
}

impl ArgPassThru {
    pub fn new(format: String, format_out: String, console: &Console) -> ArgPassThru {
        return ArgPassThru { format: format, format_out: format_out, console: console.to_owned() };
    }

    fn setformat(&mut self, format: String) {
        self.format = format;
    }

    fn setformat_out(&mut self, format: String) {
        self.format_out = format;
    }

    fn setconsole(&mut self, console: &Console) {
        self.console = console.to_owned();
    }
}

fn handleargs(args: &mut[String]) -> ArgPassThru {
    let con = Console::new(false, false, "".to_string());
    let mut pass: ArgPassThru = ArgPassThru::new("".to_string(), "".to_string(), &con);
    if args.len() > 1 {
        let mut n: usize = 0;
        for argument in args {
            if argument == "-h" || argument == "--help" {
                println!("Following arguments are passable:\n -h            Shows this message.\n -c            Re-Enter config setup.\n -format       Change the source directory date format. (Default is none)\n -format_out   Change the output date format. (Default is none)\n -dformat      Sets the format and format_out to my defaults.");
                std::process::exit(0);
            } else if argument == "-c" || argument == "--config" {
                configconstructor();
            } else if argument == "--format" || argument == "-fin" {
                let format: String = match env::args().nth(n + 1) {
                    Some(format) => format,
                    None => {
                        println!("Command line parameter after format must be passed. C[1] Exiting...");
                        std::process::exit(0);
                    },
                };
                if env::args().nth(n + 1).is_none() || format.is_empty() {
                    println!("Command line parameter after format must be passed. C[2] Exiting...");
                    std::process::exit(0);
                } else {
                    pass.setformat(format);
                }
            } else if argument == "--format_out" || argument == "-fout" {
                let format: String = match env::args().nth(n + 1) {
                    Some(format) => format,
                    None => {
                        println!("Command line parameter after format out must be passed. C[1] Exiting...");
                        std::process::exit(0);
                    },
                };
                if env::args().nth(n + 1).is_none() || format.is_empty() {
                    println!("Command line parameter after format out must be passed. C[2] Exiting...");
                    std::process::exit(0);
                } else {
                    pass.setformat_out(format);
                }
            } else if argument == "--dformat" || argument == "-df" {
                pass.setformat("%Y-%m-%d".to_string());
                pass.setformat_out("%d-%m-%Y".to_string());
            } else if argument == "-q" {
                pass.console.setquiet(true);
            } else if argument == "--timestamp" || argument == "-t" {
                let format: String = match env::args().nth(n + 1) {
                    Some(format) => format,
                    None => "%H:%M:%S".to_string(),
                };
                pass.setconsole(&Console::new(pass.console.quiet, true, format));
            }
            n += 1;
        }
    }
    return pass;
}

fn copy_files(source_dir: &Path, destination_dir: &Path, console: &Console) -> io::Result<bool> {
    let mut changed_files = false;

    if !destination_dir.exists() {
        fs::create_dir(destination_dir)?;
        console.print(format!("Missing directory {} created", destination_dir.display()));
        
    }
    if !source_dir.exists() {
        fs::create_dir(source_dir)?;
        console.print(format!("Missing directory {} created", source_dir.display()));
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
                console.print(format!("Synced file to {}", destination_path.display()));
                changed_files = true;
            }
        } else if source_path.is_dir() {
            let dir_name = source_path.file_name().unwrap();
            let destination_subdir = destination_dir.join(dir_name);
            match copy_files(&source_path, &destination_subdir, console) {
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
                console.print(format!("Deleted file at {}", destination_path.display()));
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

        match copy_files(&source_dir, &destination_dir, &argpasser.console) {
            Ok(ret) => if ret { argpasser.console.print("Files synced!".to_string()); },
            Err(error) => println!("An error occurred: {}", error),
        }

        thread::sleep(Duration::from_millis(1000));
    }
}