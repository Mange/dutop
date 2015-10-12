use std::io::{Error,ErrorKind};

pub fn describe_io_error(error: Error) -> String {
    match error.kind() {
        ErrorKind::NotFound => "File not found",
        ErrorKind::PermissionDenied => "Permission denied",
        ErrorKind::TimedOut => "Timed out",
        ErrorKind::Interrupted => "Interrupted",
        _ => "Unrecognized error"
    }.to_string()
}
