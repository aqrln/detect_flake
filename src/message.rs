use std::{io, process::Output};

#[derive(Debug)]
pub enum Message {
    ExitSuccess,
    ExitFailure {
        run_idx: u32,
        thread_idx: u32,
        output: Output,
    },
    FailedToRun(io::Error),
}
