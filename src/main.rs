use std::fs;
use std::path::{Path,PathBuf};
use std::fmt;

use EntryType::{Directory,File};

#[derive(Debug)]
enum EntryType {
    Directory,
    File,
}

type EntryList = Vec<Entry>;

struct Entry {
    entry_type: EntryType,
    path: PathBuf,
    self_size: u64,
}

impl Entry {
    fn from_metadata(path: PathBuf, metadata: &fs::Metadata) -> Option<Entry> {
        if metadata.is_dir() {
            Some(Entry {path: path, entry_type: Directory, self_size: metadata.len()})
        } else if metadata.is_file() {
            Some(Entry {path: path, entry_type: File, self_size: metadata.len()})
        } else {
            None
        }
    }

    fn for_path(path: PathBuf) -> Option<Entry> {
        match fs::metadata(&path) {
            Ok(metadata) => Entry::from_metadata(path, &metadata),
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
        // TODO: Cache this
        match self.entry_type {
            Directory => Entry::in_directory(&self.path),
            File => Vec::new(),
        }
    }

    fn size(&self) -> u64 {
        self.self_size + self.descendent_size()
    }

    fn descendent_size(&self) -> u64 {
        self.children().into_iter().
            map(|child| child.size() ).fold(0, |a, n| a + n)
    }

    fn file_name(&self) -> &str {
        // Converting paths to strings might fail. We start by trying to convert the
        // filename. If the path is something like ".", it will fail as there is no "file
        // name" in that path. We fall back to the entire path in that case. If that also
        // fails, we fall back to hardcoded representation.
        let file_name = self.path.file_name().and_then(|s| s.to_str());
        file_name.or_else(|| self.path.to_str()).unwrap_or("(no name)")
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

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.file_name(), self.size().as_size_display())
    }
}

fn print_tree(dir_entry: Entry) {
    print_indented_tree(dir_entry, 0);
}

fn print_indented_tree(entry: Entry, indent: usize) {
    println!("{0:1$}{2}", "", indent * 2, entry);
    for child in entry.children() {
        print_indented_tree(child, indent + 1);
    }
}

fn main() {
    let root = Entry::for_path(PathBuf::from(".")).unwrap();
    print_tree(root);
}
