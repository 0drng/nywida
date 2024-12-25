use args::Args;
use clap::Parser;
use error::ApplicationError;
use model::{ConfigFile, ConfigFileLock, DotConfig, Script};
use service::{
    config_service,
    translation_service::{t, Labels},
};
use wrapper::command;

mod args;
mod error;
mod model;
mod service;
mod wrapper;

fn main() -> Result<(), ApplicationError> {
    let args: Args = Args::parse();
    
    let uid: u32 = command::get_uid();
    if uid != 0 {
        eprintln!("{}", t(Labels::Error_NoRoot, None));
        std::process::exit(1);
    }

    let config_file: ConfigFile = config_service::get_config(&args.config_path);
    let config_file_lock_pre: ConfigFileLock = config_service::get_config_file_lock();
    let mut config_file_lock: ConfigFileLock = ConfigFileLock::empty();

    command::ask_continue()?;

    let scripts_path: Vec<Script> = config_service::get_pre_scripts(&config_file);
    let mut runned_pre_scripts: Vec<Script> = Vec::new();
    for script_path in scripts_path {
        println!(
            "{}",
            t(
                Labels::Info_ExecutingPreScript,
                Option::Some(vec![script_path.bin.clone()])
            )
        );
        command::run_command("sh", vec![script_path.bin.clone()])?;
        runned_pre_scripts.push(script_path);
    }
    config_file_lock.pre_scripts_runned.append(&mut runned_pre_scripts);



    let amount: usize = config_service::install_all_packages(&config_file, args.update)?;
    println!(
        "{}",
        t(
            Labels::Info_NewlyInstalledPackages,
            Option::Some(vec![amount.to_string()])
        )
    );    
    let installed_packages: Vec<String> = config_service::get_packages_str(&config_file, None, Option::Some(true));
    config_file_lock.packages_installed = installed_packages;

    let amount: usize = config_service::remove_all_packages(&config_file_lock_pre, &config_file, args.dependencies)?;
    println!(
        "{}",
        t(
            Labels::Info_NewlyUninstalledPackages,
            Option::Some(vec![amount.to_string()])
        )
    );
    let removed_packages: Vec<String> = config_service::get_packages_str(&config_file, None, Option::Some(false));
    config_file_lock.packages_removed = removed_packages;

    let dot_files: Vec<DotConfig> = config_file
        .packages
        .iter()
        .filter_map(|f| f.get_dot_config())
        .collect();

    for dot_file in dot_files {
        if let Some(src) = dot_file.src {
            config_service::copy_file(&src, &dot_file.dest).unwrap();
        }
        if let Some(content) = dot_file.content {
            std::fs::write(&dot_file.dest, content).unwrap();
        }
    }

    let scripts_path: Vec<Script> = config_service::get_post_scripts(&config_file);
    let mut runned_post_scripts: Vec<Script> = Vec::new();

    for script_path in scripts_path {
        println!(
            "{}",
            t(
                Labels::Info_ExecutingPostScript,
                Option::Some(vec![script_path.bin.clone()])
            )
        );
        command::run_command("sh", vec![script_path.bin.clone()])?;
        runned_post_scripts.push(script_path);
    }
    config_file_lock.post_scripts_runned.append(&mut runned_post_scripts);

    config_service::save_lock(&config_file_lock);

    Ok(())
}
