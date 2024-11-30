use args::Args;
use clap::Parser;
use error::ApplicationError;
use model::{ConfigFile, DotConfig};
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

    command::ask_continue()?;

    let scripts_path: Vec<String> = config_service::get_before_scripts(&config_file);

    for script_path in scripts_path {
        println!(
            "{}",
            t(
                Labels::Info_ExecutingPreScript,
                Option::Some(vec![script_path.clone()])
            )
        );
        command::run_command("sh", vec![script_path])?;
    }

    let amount: usize = config_service::install_all_packages(&config_file, args.update)?;
    println!(
        "{}",
        t(
            Labels::Info_NewlyInstalledPackages,
            Option::Some(vec![amount.to_string()])
        )
    );

    let amount: usize = config_service::remove_all_packages(&config_file, args.dependencies)?;
    println!(
        "{}",
        t(
            Labels::Info_NewlyUninstalledPackages,
            Option::Some(vec![amount.to_string()])
        )
    );

    let dot_files: Vec<DotConfig> = config_file
        .packages
        .iter()
        .filter_map(|f| f.get_dot_config())
        .collect();

    for dot_file in dot_files {
        config_service::copy_file(&dot_file.src, &dot_file.dest).unwrap();
    }

    let scripts_path: Vec<String> = config_service::get_post_scripts(&config_file);

    for script_path in scripts_path {
        println!(
            "{}",
            t(
                Labels::Info_ExecutingPostScript,
                Option::Some(vec![script_path.clone()])
            )
        );
        command::run_command("sh", vec![script_path])?;
    }

    Ok(())
}
