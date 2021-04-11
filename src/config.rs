use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct LinesConfig {
  /// Whether lines only consisting of a comment should be excluded
  #[structopt(short = "ic", long)]
  pub include_comments: bool,

  /// Whether empty files will be excluded
  #[structopt(long)]
  pub ignore_empty: bool,

  /// Whether the progress bar for the directory mode is disabled
  #[structopt(long)]
  pub no_progressbar: bool
}
