pub use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    about = "Run many instances of the same command in parallel to find abnormal behavior or check if a test is flaky."
)]
pub struct Opt {
    /// Command to run
    #[clap(short, long)]
    pub command: String,

    /// Number of parallel threads
    #[clap(short, long, default_value = "10")]
    pub threads: u32,

    /// Number of serial runs per each thread
    #[clap(short, long, default_value = "100")]
    pub runs_per_thread: u32,

    /// Inherit stdio instead of redirecting to /dev/null
    #[clap(short, long)]
    pub inherit_stdio: bool,

    /// Print the stdout and stderr of unsucessful runs only
    #[clap(short, long, conflicts_with = "inherit_stdio")]
    pub print_failing_output: bool,
}
