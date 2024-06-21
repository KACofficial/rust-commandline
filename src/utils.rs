// use rustyline::config::Configurer;
//use std::io;
use rustyline::{DefaultEditor, Result};
use std::fs::File;
use std::io::BufReader;
use serde::Deserialize;
use serde_json;
// use rustyline::Context;
//use rustyline::history;

#[derive(Deserialize)]
pub struct Config {
    pub host_name: String,
    pub user_name: String,

}

pub fn main_prompt_setup(h_fn: &str) -> Result<DefaultEditor> {
    let mut rl = DefaultEditor::new()?;
    if rl.load_history(h_fn).is_err() {
        println!("Previous history unavailable: {}", h_fn);
    }
    Ok(rl)
}

pub fn main_config_setup(c_fn: &str) -> std::result::Result<Config, Box<dyn std::error::Error>> {
    match read_config_file(c_fn) {
        Ok(config) => Ok(Config {
                host_name: config.host_name,
                user_name: config.user_name 
            }),
        Err(e) => {
            println!("Error reading JSON file: {}", e);
            Err(Box::new(e))
        },
    }
}

pub fn main_cleanup(h_fn: &str, rl: &mut DefaultEditor) -> Result<()> {
    let _ = rl.save_history(h_fn);
    Ok(())
}

fn read_config_file(file_path: &str) -> std::result::Result<Config, std::io::Error> {
    // Open the file in read-only mode with buffer.
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Parse the JSON file.
    let config = serde_json::from_reader(reader)?;

    Ok(config)
}