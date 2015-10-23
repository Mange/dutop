use std::path::Path;
use std::io::{Error,ErrorKind};

pub fn full_name_from_path(path: &Path, is_dir: bool) -> String {
    let name = path.to_string_lossy().into_owned();
    add_slash_if_needed(name, is_dir)
}

pub fn short_name_from_path(path: &Path, is_dir: bool) -> String {
    match path.file_name() {
        Some(file_name) => {
            let name = file_name.to_string_lossy().into_owned();
            add_slash_if_needed(name, is_dir)
        },
        None => full_name_from_path(path, is_dir)
    }
}

fn add_slash_if_needed(string: String, is_dir: bool) -> String {
    match (string.ends_with("/"), is_dir) {
        (true, true) | (false, false) => string,
        (false, true) => string + "/",
        (true, false) => {
            let mut string = string;
            string.pop();
            string
        },
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn it_can_convert_a_path_to_a_string() {
        let path = Path::new("/path/to");

        assert_eq!(full_name_from_path(&path, false), "/path/to");
        assert_eq!(full_name_from_path(&path, true), "/path/to/");
    }

    #[test]
    fn it_can_convert_a_filename_to_a_string() {
        let path = Path::new("/path/to");

        assert_eq!(short_name_from_path(&path, false), "to");
        assert_eq!(short_name_from_path(&path, true), "to/");
    }

    #[test]
    fn it_handles_ending_slash_already_present() {
        let path = Path::new("/path/to/");

        // Strip slash of file paths
        assert_eq!(short_name_from_path(&path, false), "to");
        assert_eq!(full_name_from_path(&path, false), "/path/to");

        // Don't add an extra slash on directories
        assert_eq!(short_name_from_path(&path, true), "to/");
        assert_eq!(full_name_from_path(&path, true), "/path/to/");
    }

    #[test]
    fn it_can_format_sizes() {
        assert_eq!(          1.as_size_display(),       "1 B");
        assert_eq!(        345.as_size_display(),     "345 B");
        assert_eq!(       1000.as_size_display(),   "1.00 kB");
        assert_eq!(       1100.as_size_display(),   "1.10 kB");
        assert_eq!(      11000.as_size_display(),  "11.00 kB");
        assert_eq!(123_456_789.as_size_display(), "123.46 MB");
        assert_eq!(123_452_000.as_size_display(), "123.45 MB");

        assert_eq!(    867_000_000_000.as_size_display(),    "867.00 GB");
        assert_eq!(867_000_000_000_000.as_size_display(), "867000.00 GB");
    }
}
