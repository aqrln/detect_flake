pub use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    about = "Run many instances of the same command in parallel to find abnormal behavior or check if a test is flaky."
)]
pub struct Opt {
    /// Command to run
    #[structopt(short, long)]
    pub command: String,

    /// Number of parallel threads
    #[structopt(short, long, default_value = "100")]
    pub threads: u32,

    /// Number of serial runs per each thread
    #[structopt(short, long, default_value = "1")]
    pub runs_per_thread: u32,

    /// Inherit stdio instead of redirecting to /dev/null
    #[structopt(short, long)]
    pub inherit_stdio: bool,
}

impl Opt {
    pub fn threads_as_u64(&self) -> u64 {
        self.threads.into()
    }

    pub fn runs_per_thread_as_u64(&self) -> u64 {
        self.runs_per_thread.into()
    }
}
