mod command;
mod message;
mod opt;
mod runner;

use std::sync::mpsc;

use indicatif::ProgressBar;

use crate::{
    message::Message,
    opt::{Opt, StructOpt},
};

fn main() {
    let opt = Opt::from_args();

    let (tx, rx) = mpsc::channel();
    let handles = runner::start_threads(&opt, tx);

    let total_runs = opt.threads_as_u64() * opt.runs_per_thread_as_u64();
    let mut count_success: u64 = 0;
    let mut count_failure: u64 = 0;

    let bar = ProgressBar::new(total_runs);

    for msg in rx {
        match msg {
            Message::ExitStatusSuccess => {
                count_success += 1;
            }
            Message::ExitStatusFailure => {
                count_failure += 1;
            }
            Message::FailedToRun(error) => {
                count_failure += 1;
                bar.println(error);
            }
        }
        bar.inc(1);
    }

    bar.finish();

    println!("Success: {}/{}", count_success, total_runs);
    println!("Failure: {}/{}", count_failure, total_runs);

    runner::join_threads(handles);
}
