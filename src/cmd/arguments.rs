use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Config file path
    #[arg(short, long, default_value = "/etc/rustysearch/config.json")]
    config: String,

    /// Change the log level
    #[arg(short = 'l', long, default_value = "info")]
    loglevel: String,

    /// Change Database path
    #[arg(short = 'D', long, default_value = "/tmp/rustysearch.db")]
    database: String,
}
