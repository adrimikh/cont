
pub struct Args {
  #[structopt(short, long)]
  debug: bool,
}

pub fn parse_args() -> Args {
  let args = Args::from_args();

} 	