use std::fs;
use std::path::{Path,PathBuf};

use Entry::{Directory,File};

#[derive(Debug)]
enum Entry {
    Directory(PathBuf),
    File(PathBuf, u64),
}

impl Entry {
    fn for_path(path: PathBuf) -> Option<Entry> {
        match fs::metadata(&path) {
            Ok(ref metadata) if metadata.is_dir() => Some(Directory(path)),
            Ok(ref metadata) if metadata.is_file() => Some(File(path, metadata.len())),
            _ => None
        }
    }

    fn in_directory(dir: &Path) -> Vec<Entry> {
        match fs::read_dir(dir) {
            Ok(read_dir) => {
                read_dir.filter_map(|child| {
                    match child {
                        Ok(child) => Entry::for_path(child.path()),
                        Err(..) => None,
                    }
                }).collect()
            },
            Err(..) => Vec::new()
        }
    }

    fn children(&self) -> Vec<Entry> {
        match *self {
            Directory(ref dir) => Entry::in_directory(&dir),
            File(..) => Vec::new(),
        }
    }

    fn size(&self) -> u64 {
        match *self {
            Directory(_) =>
                self.children().into_iter().
                    map(|child| child.size() ).fold(0, |a, n| a + n),
            File(_, size) => size,
        }
    }
}

trait SizeDisplay {
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

fn main() {
    let root = Entry::for_path(PathBuf::from(".")).unwrap();
    println!("{}", root.size().as_size_display());
}
