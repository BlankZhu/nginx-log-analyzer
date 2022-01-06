use clap::{AppSettings, Clap};

#[derive(Clap, Debug)]
#[clap(version = "0.1", author = "BlankZhu")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Option {
    #[clap(short, long, default_value = "", about = "ryna's config file")]
    pub config_filename: String,

    #[clap(short, long, default_value = "", about = "nginx's access log file")]
    pub access_log_filename: String,
}
