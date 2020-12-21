use clap_verbosity_flag::Verbosity;
use exitfailure::ExitFailure;
use failure::ResultExt;
use grex;
use log::info;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::path;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt, Debug)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: path::PathBuf,
    #[structopt(flatten)]
    verbose: Verbosity,
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    args.verbose.setup_env_logger(&env!("CARGO_PKG_NAME"))?;

    info!("Looking for {} in {:?}", args.pattern, args.path);

    let f = fs::File::open(&args.path)
        .with_context(|_| format!("could not read file {:?}", &args.path))?;

    let lines = BufReader::new(f).lines().filter_map(Result::ok);
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for line in grex::find_matches(lines, &args.pattern) {
        write!(handle, "{}\n", line)?;
    }
    Ok(())
}