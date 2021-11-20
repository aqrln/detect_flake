use std::{
    process::{Command, Stdio},
    sync::mpsc::Sender,
    thread::{self, JoinHandle},
};

use crate::{command::CommandParser, message::Message, opt::Opt};

pub fn start_threads(opt: &Opt, tx: Sender<Message>) -> Vec<JoinHandle<()>> {
    let cmd = CommandParser::from_command(&opt.command);

    let handles = (0..opt.threads).map(|_| {
        let (program, args) = cmd.to_parts_owned();
        let runs_per_thread = opt.runs_per_thread;
        let inherit_stdio = opt.inherit_stdio;
        let tx = tx.clone();

        thread::spawn(move || {
            let get_stdio = || {
                if inherit_stdio {
                    Stdio::inherit()
                } else {
                    Stdio::null()
                }
            };

            for _ in 0..runs_per_thread {
                let msg = match Command::new(&program)
                    .args(&args)
                    .stdout(get_stdio())
                    .stderr(get_stdio())
                    .status()
                {
                    Ok(status) => {
                        if status.success() {
                            Message::ExitStatusSuccess
                        } else {
                            Message::ExitStatusFailure
                        }
                    }
                    Err(error) => Message::FailedToRun(error.to_string()),
                };
                tx.send(msg).unwrap();
            }
        })
    });

    handles.collect()
}

pub fn join_threads(threads: Vec<JoinHandle<()>>) {
    threads
        .into_iter()
        .for_each(|handle| handle.join().unwrap());
}
