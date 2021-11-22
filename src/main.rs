mod command;
mod message;
mod opt;
mod runner;

use indicatif::ProgressBar;

use crate::{
    message::Message,
    opt::{Opt, StructOpt},
};

fn main() {
    let opt = Opt::from_args();

    let (tx, rx) = crossbeam_channel::unbounded();
    let handles = runner::start_threads(&opt, tx);

    let total_count = opt.threads_as_u64() * opt.runs_per_thread_as_u64();
    let mut success_count = 0u64;
    let mut failure_count = 0u64;

    let bar = ProgressBar::new(total_count);
    bar.tick();

    for msg in rx {
        match msg {
            Message::ExitStatusSuccess => {
                success_count += 1;
            }
            Message::ExitStatusFailure => {
                failure_count += 1;
            }
            Message::FailedToRun(error) => {
                failure_count += 1;
                bar.println(error);
            }
        }
        bar.inc(1);
    }

    bar.finish();

    println!("Success: {}/{}", success_count, total_count);
    println!("Failure: {}/{}", failure_count, total_count);

    runner::join_threads(handles);
}
