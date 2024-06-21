mod cmd_p;
mod utils;

use rustyline::error::ReadlineError;
use rustyline::Result;
use cmd_p::process_command;
use std::env;
use std::path::PathBuf;
use utils::*;
use colored::*;


fn main() -> Result<()> {
    // SETUP
    let historyfilename = "data/history.txt";
    let configfilename = "data/config/main_conf.json";
    let mut rl = main_prompt_setup(&historyfilename)?;
   // let config_data = main_config_setup(&configfilename);\
   let prompt: String;
   match main_config_setup(&configfilename) {
       Ok(config) => {
           prompt = format!("{}@{}> ", config.user_name.bold().cyan(), config.host_name.bold().green());
       },
       Err(e) => {
           println!("Error reading JSON file: {}", e);
           println!("Using default prompt.");
           prompt = "RCL> ".to_string();

       },
   }
    loop {
        let current_dir: PathBuf = env::current_dir()?;
        let c_dir = current_dir.display().to_string();
        println!("{}", c_dir.yellow());
        let readline = rl.readline(&prompt);
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                if line == "" { continue; }
                let code = process_command(line.as_str());
                if code == 1 {
                    println!("Error in command: {}", line);
                    continue;
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    let _ = main_cleanup(&historyfilename, &mut rl);
    Ok(())
}

