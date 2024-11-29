use args::Args;
use clap::Parser;
use error::ApplicationError;
use model::{ConfigFile, DotConfig};
use wrapper::command;
use service::config_service;

mod args;
mod error;
mod service;
mod model;
mod wrapper;

fn main() -> Result<(), ApplicationError>{   

    let uid: u32 = command::get_uid();
    if uid != 0 {
        eprintln!("The programm should be executed with root. Try to use sudo/doas.");
        std::process::exit(1);
    }

    println!(r"  
  _   ___     ____          _______ _____          
 | \ | \ \   / /\ \        / /_   _|  __ \   /\    
 |  \| |\ \_/ /  \ \  /\  / /  | | | |  | | /  \   
 | . ` | \   /    \ \/  \/ /   | | | |  | |/ /\ \  
 | |\  |  | |      \  /\  /   _| |_| |__| / ____ \ 
 |_| \_|  |_|       \/  \/   |_____|_____/_/    \_\
                                                   ");
    command::ask_continue()?;

    let args: Args = Args::parse();

    let config_file: ConfigFile = config_service::get_config(&args.config_path);

    let scripts_path: Vec<String> = config_service::get_before_scripts(&config_file);

    for script_path in scripts_path {
        println!("Running before script: {}", script_path);
        command::run_command("sh", vec![script_path]).unwrap();
    }
    
    let amount: usize = config_service::install_all_packages(&config_file, args.update)?;
    println!("Re/Installed {} packages", amount);

    let dot_files: Vec<DotConfig> = config_file.packages.iter().filter_map(|f| f.dot_config.clone()).collect();
    
    for dot_file in dot_files {
        config_service::copy_file(&dot_file.src, &dot_file.dest).unwrap();
    }

    let scripts_path: Vec<String> = config_service::get_post_scripts(&config_file);

    for script_path in scripts_path {
        println!("Running post script: {}", script_path);
        command::run_command("sh", vec![script_path]).unwrap();
    }

    Ok(())
}