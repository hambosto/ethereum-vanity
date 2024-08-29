use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    #[structopt(short, long, default_value = "0")]
    pub workers: usize,

    #[structopt(parse(from_os_str), short, long)]
    pub input: Option<PathBuf>,

    #[structopt(short, long)]
    pub prefix: Option<String>,

    #[structopt(short, long)]
    pub suffix: Option<String>,
}
