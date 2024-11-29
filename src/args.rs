use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "/etc/nywida/configuration.jsonc")]
    pub config_path: String,

    #[arg(short, long)]
    pub update: bool,
}