use std::{
    io::{self, Write},
    process::{Command, Stdio},
    thread::{self, JoinHandle},
};

use crossbeam_channel::Sender;

use crate::{command::CommandParser, message::Message, opt::Opt};

pub fn start_threads(opt: &Opt, tx: Sender<Message>) -> Vec<JoinHandle<()>> {
    let cmd = CommandParser::from_command(&opt.command);

    let handles = (0..opt.threads).map(|thread_idx| {
        let (program, args) = cmd.to_parts_owned();
        let runs_per_thread = opt.runs_per_thread;
        let inherit_stdio = opt.inherit_stdio;
        let print_failing_output = opt.print_failing_output;
        let tx = tx.clone();

        thread::spawn(move || {
            let get_stdio = || {
                if inherit_stdio {
                    Stdio::inherit()
                } else if print_failing_output {
                    Stdio::piped()
                } else {
                    Stdio::null()
                }
            };

            for i in 1..=runs_per_thread {
                let msg = match Command::new(&program)
                    .args(&args)
                    .stdout(get_stdio())
                    .stderr(get_stdio())
                    .output()
                {
                    Ok(output) => {
                        if output.status.success() {
                            Message::ExitStatusSuccess
                        } else {
                            if print_failing_output {
                                println!("----------------------------------------");
                                println!("Run {i} in thread {thread_idx} stdout");
                                println!("----------------------------------------");
                                io::stdout().write_all(&output.stdout).unwrap();
                                eprintln!("----------------------------------------");
                                eprintln!("Run {i} in thread {thread_idx} stderr");
                                eprintln!("----------------------------------------");
                                io::stderr().write_all(&output.stderr).unwrap();
                            }
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
