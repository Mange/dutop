use std::path::PathBuf;
use std::io::{Error,ErrorKind};

pub fn full_name_from_path(path: &PathBuf, is_dir: bool) -> String {
    let name = path.to_string_lossy().into_owned();
    if is_dir {
        name + "/"
    } else {
        name
    }
}

pub fn short_name_from_path(path: &PathBuf, is_dir: bool) -> String {
    match path.file_name() {
        Some(file_name) => {
            let name = file_name.to_string_lossy().into_owned();
            if is_dir {
                name + "/"
            } else {
                name
            }
        },
        None => full_name_from_path(path, is_dir)
    }
}

pub fn describe_io_error(error: Error) -> String {
    match error.kind() {
        ErrorKind::NotFound => "File not found",
        ErrorKind::PermissionDenied => "Permission denied",
        ErrorKind::TimedOut => "Timed out",
        ErrorKind::Interrupted => "Interrupted",
        _ => "Unrecognized error"
    }.to_string()
}

pub trait SizeDisplay {
    fn as_size_display(&self) -> String;
}

const KILO: f64 = 1e3 as f64;
const MEGA: f64 = 1e6 as f64;
const GIGA: f64 = 1e9 as f64;

const KILO_CUTOFF: u64 = (0.6 * KILO) as u64;
const MEGA_CUTOFF: u64 = (1.4 * MEGA) as u64;
const GIGA_CUTOFF: u64 = (1.4 * GIGA) as u64;

impl SizeDisplay for u64 {
    fn as_size_display(&self) -> String {
        if *self < KILO_CUTOFF {
            return format!("{} B", *self)
        }

        let (scaled, unit) = match *self {
            KILO_CUTOFF...MEGA_CUTOFF => (*self as f64 / KILO, "kB"),
            MEGA_CUTOFF...GIGA_CUTOFF => (*self as f64 / MEGA, "MB"),
            _ => (*self as f64 / GIGA, "GB"),
        };
        format!("{:.2} {}", scaled, unit)
    }
}

