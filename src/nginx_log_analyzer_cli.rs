use clap::{AppSettings, Clap};

#[derive(Clap)]
#[clap(version = "0.1", author = "BlankZhu")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct NginxLogAnalyzerCli {
    #[clap(short, long, default_value = "res/log.fmt", about = "log format file")]
    pub logfmt: String,

    #[clap(short, long, default_value = "res/typ.fmt", about = "type format file")]
    pub typfmt: String,

    #[clap(short, long, default_value = "res/access.log", about = "access log file")]
    pub acclog: String,
}

