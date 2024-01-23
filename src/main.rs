mod command;
mod message;
mod opt;
mod runner;

use indicatif::ProgressBar;
use std::{
    io::{self, Write},
    process,
};

use crate::{
    command::ParsedCommand,
    message::Message,
    opt::{Opt, Parser},
};

fn main() -> anyhow::Result<()> {
    let opt = Opt::parse();
    let cmd = ParsedCommand::parse(&opt.command)?;

    let (tx, rx) = crossbeam_channel::unbounded();
    let handles = runner::start_threads(&opt, &cmd, tx);

    let total_count = (opt.threads as u64) * (opt.runs_per_thread as u64);
    let mut success_count = 0u64;
    let mut failure_count = 0u64;

    let bar = ProgressBar::new(total_count);
    bar.tick();

    for msg in rx {
        match msg {
            Message::ExitSuccess => {
                success_count += 1;
            }

            Message::ExitFailure {
                run_idx,
                thread_idx,
                output,
            } => {
                if opt.print_failing_output {
                    println!("----------------------------------------");
                    println!("Run {run_idx} in thread {thread_idx} stdout");
                    println!("----------------------------------------");
                    io::stdout().write_all(&output.stdout).unwrap();
                    eprintln!("----------------------------------------");
                    eprintln!("Run {run_idx} in thread {thread_idx} stderr");
                    eprintln!("----------------------------------------");
                    io::stderr().write_all(&output.stderr).unwrap();
                }

                if opt.exit_early_on_error {
                    process::exit(output.status.code().unwrap_or(1));
                } else {
                    failure_count += 1;
                }
            }

            Message::FailedToRun(error) => {
                bar.println(error.to_string());

                if opt.exit_early_on_error {
                    process::exit(1);
                } else {
                    failure_count += 1;
                }
            }
        }

        bar.inc(1);
    }

    bar.finish();

    println!("Success: {}/{}", success_count, total_count);
    println!("Failure: {}/{}", failure_count, total_count);

    runner::join_threads(handles);

    if failure_count > 0 {
        Err(anyhow::anyhow!("some runs failed"))
    } else {
        Ok(())
    }
}
