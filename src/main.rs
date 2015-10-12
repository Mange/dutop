#[macro_use]
extern crate clap;

use std::fs;
use std::path::{Path,PathBuf};
use std::fmt;
use std::slice::Iter;

mod arguments;

struct Entry {
    path: PathBuf,
    self_size: u64,
    children: Vec<Entry>,
}

impl Entry {
    fn from_metadata(path: PathBuf, metadata: &fs::Metadata) -> Option<Entry> {
        let children = if metadata.is_dir() {
            Entry::in_directory(&path)
        } else if metadata.is_file() {
            vec![]
        } else {
            return None;
        };

        Some(Entry {children: children, path: path, self_size: metadata.len()})
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

    fn descendent_size(&self) -> u64 {
        self.children.iter().map(|child| child.size()).fold(0, |a, n| a + n)
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

trait DisplayableEntry : fmt::Display + Sized {
    fn size(&self) -> u64;
    fn name(&self) -> &str;
    fn children_iter(&self) -> Iter<Self>;
}

impl DisplayableEntry for Entry {
    fn size(&self) -> u64 {
        self.self_size + self.descendent_size()
    }

    fn name(&self) -> &str {
        // Converting paths to strings might fail. We start by trying to convert the
        // filename. If the path is something like ".", it will fail as there is no "file
        // name" in that path. We fall back to the entire path in that case. If that also
        // fails, we fall back to hardcoded representation.
        let file_name = self.path.file_name().and_then(|s| s.to_str());
        file_name.or_else(|| self.path.to_str()).unwrap_or("(no name)")
    }

    fn children_iter(&self) -> Iter<Entry> {
        self.children.iter()
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.name(), self.size().as_size_display())
    }
}

fn print_tree<T: DisplayableEntry>(entry: &T) {
    print_indented_tree(entry, 0);
}

fn print_indented_tree<T: DisplayableEntry>(entry: &T, indent: usize) {
    println!("{0:1$}{2}", "", indent * 2, entry);
    for child in entry.children_iter() {
        print_indented_tree(child, indent + 1);
    }
}

fn main() {
    let options = arguments::parse();
    match Entry::for_path(options.root()) {
        Some(root) => print_tree(&root),
        None =>
            println!(
                "Cannot open {} for reading. Does it exist, and do you have permission to open it?",
                options.root().to_string_lossy()
            )
    }
}
