use serde::{Serialize, Deserialize};
use std::path::Path;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::Duration;
use std::thread;

pub fn read_input(prompt: &str) -> String {
    use std::io::{self};
    let mut buffer: String = String::new();
    print!("{} ", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_owned()
}

#[derive(Serialize, Deserialize)]
pub struct ConfigStruct {
    pub source_dir: String,
    pub base_out_dir: String,
}

pub fn configconstructor() -> ConfigStruct {
    println!("Config setup entered!");
    let srcinput = read_input("Source Directory Path: ");
    
    let input = read_input("Sync Directory Path: ");
    let jsonstring = format!(r#"{{"source_dir": "{}", "base_out_dir": "{}"}}"#, srcinput, input);
    saveconfig(jsonstring);
    thread::sleep(Duration::from_secs(1));
    return getconfig();
}

pub fn saveconfig(configstr: String) {
    let bytestr = configstr.as_bytes();
    let mut cwriter = OpenOptions::new()
                                .read(true)
                                .write(true)
                                .create(true)
                                .open("fsync.config")
                                .unwrap();
    cwriter.write_all(bytestr).expect("Unable to write config!");
}

pub fn hasconfig() -> bool {
    return Path::new("fsync.config").exists();
}

pub fn getconfig() -> ConfigStruct {
    if hasconfig() {
        let data = fs::read_to_string("fsync.config").expect("Failed to read config file!");
        let config: ConfigStruct = serde_json::from_str(data.as_str()).expect("Unable to parse config file!");
        return config;
    } else {
        return configconstructor();
    }
}