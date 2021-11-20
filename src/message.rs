#[derive(Debug)]
pub enum Message {
    ExitStatusSuccess,
    ExitStatusFailure,
    FailedToRun(String),
}
