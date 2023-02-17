#[derive(Debug)]
pub enum Message {
    ExitStatusSuccess,
    ExitStatusFailure(Option<i32>),
    FailedToRun(String),
}
