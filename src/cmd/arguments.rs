use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Config file path
    #[arg(short, long, default_value = "/etc/rustysearch/config.json")]
    pub config_path: String,

    /// Change the log level
    #[arg(short = 'l', long, default_value = "info")]
    pub log_level: String,

    /// Change Database path
    #[arg(short = 'D', long, default_value = "/tmp/rustysearch.db")]
    pub database_path: String,
}
