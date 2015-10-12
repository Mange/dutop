#[macro_use]
extern crate clap;

use std::fs;
use std::path::{Path,PathBuf};
use std::fmt;
use std::slice::Iter;

mod arguments;
mod utils;

use utils::SizeDisplay;

struct Entry {
    path: PathBuf,
    self_size: u64,
    children: Vec<Entry>,
}

impl Entry {
    fn from_metadata(path: PathBuf, metadata: &fs::Metadata) -> Result<Entry, String> {
        let children = if metadata.is_dir() {
            Entry::in_directory(&path)
        } else if metadata.is_file() {
            vec![]
        } else {
            return Err("not a file or directory".to_string());
        };

        Ok(Entry {children: children, path: path, self_size: metadata.len()})
    }

    fn for_path(path: PathBuf) -> Result<Entry, String> {
        match fs::metadata(&path) {
            Ok(metadata) => Entry::from_metadata(path, &metadata),
            Err(error) => Err(utils::describe_io_error(error))
        }
    }

    fn in_directory(dir: &Path) -> Vec<Entry> {
        match fs::read_dir(dir) {
            Ok(read_dir) => {
                read_dir.filter_map(|child| {
                    match child {
                        Ok(child) => Entry::for_path(child.path()).ok(),
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

trait DisplayableEntry : fmt::Display + Sized {
    type Child: DisplayableEntry;

    fn size(&self) -> u64;
    fn name(&self) -> &str;
    fn children_iter(&self) -> Iter<Self::Child>;
}

impl DisplayableEntry for Entry {
    type Child = Entry;

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

struct Root {
    entry: Entry,
}

impl Root {
    fn new(entry: Entry) -> Root {
        Root{entry: entry}
    }
}

impl DisplayableEntry for Root {
    type Child = Entry;

    fn size(&self) -> u64 {
        self.entry.size()
    }

    fn name(&self) -> &str {
        self.entry.path.to_str().unwrap_or("(no name)")
    }

    fn children_iter(&self) -> Iter<Entry> {
        self.entry.children_iter()
    }
}

impl fmt::Display for Root {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.name(), self.entry.size().as_size_display())
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
    for root in options.roots() {
        match Entry::for_path(root.clone()) {
            Ok(root) => print_tree(&Root::new(root)),
            Err(message) =>
                println!("{} ERROR: {}", root.to_string_lossy(), message)
        }
    }
}
