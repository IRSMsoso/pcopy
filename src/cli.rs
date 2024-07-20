use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Args {
    /// Input file or directory
    pub(crate) input: String,

    /// Output file or directory
    pub(crate) output_dir: String,

    /// Whether to overrite the files there
    #[arg(short, long, default_value_t = false)]
    pub(crate) force: bool,
}
