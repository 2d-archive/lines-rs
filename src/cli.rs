use std::path::PathBuf;
use structopt::StructOpt;
use crate::config::LinesConfig;

#[derive(StructOpt, Debug)]
#[structopt(name = "lines")]
pub struct Cli {
  #[structopt(flatten)]
  pub config: LinesConfig,
  pub path: PathBuf,
}

pub fn parse() -> Cli {
  Cli::from_args()
}
