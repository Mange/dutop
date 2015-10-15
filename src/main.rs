#[macro_use]
extern crate clap;

use std::fs;
use std::path::{Path,PathBuf};
use std::fmt;
use std::slice::Iter;

mod arguments;
mod utils;

use arguments::Options;
use utils::SizeDisplay;

struct Entry {
    name: String,
    self_size: u64,
    children: Vec<Entry>,
}

impl Entry {
    fn from_metadata(path: PathBuf, metadata: &fs::Metadata) -> Result<Entry, String> {
        let mut children = if metadata.is_dir() {
            Entry::in_directory(&path)
        } else if metadata.is_file() {
            vec![]
        } else {
            return Err("not a file or directory".to_string());
        };

        children.sort_by(
            // Note: We change the ordering to get in descending order
            |a, b| b.size().cmp(&a.size())
        );

        Ok(Entry {
            name: utils::short_name_from_path(&path, metadata.is_dir()),
            children: children,
            self_size: metadata.len()
        })
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
    fn name(&self) -> &String;
    fn children_iter(&self) -> Iter<Self::Child>;

    fn is_hidden(&self) -> bool {
        self.name().chars().nth(0) == Some('.')
    }
}

impl DisplayableEntry for Entry {
    type Child = Entry;

    fn size(&self) -> u64 {
        self.self_size + self.descendent_size()
    }

    fn name(&self) -> &String {
        &self.name
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
    name: String,
    entry: Entry,
}

impl Root {
    fn for_path(path: PathBuf) -> Result<Root, String> {
        match fs::metadata(&path) {
            Ok(metadata) => Root::from_metadata(path, &metadata),
            Err(error) => Err(utils::describe_io_error(error))
        }
    }

    fn from_metadata(path: PathBuf, metadata: &fs::Metadata) -> Result<Root, String> {
        Entry::from_metadata(path.clone(), &metadata).map(|entry| {
            Root{
                name: utils::full_name_from_path(&path, true),
                entry: entry,
            }
        })
    }
}

impl DisplayableEntry for Root {
    type Child = Entry;

    fn size(&self) -> u64 {
        self.entry.size()
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn children_iter(&self) -> Iter<Entry> {
        self.entry.children_iter()
    }

    fn is_hidden(&self) -> bool {
        // Roots are never hidden; we always want to show them since the user gave them to us
        // explicitly.
        // Roots can also have the name ".", so they would appear to be hidden in that case unless
        // we handle it differently.
        false
    }
}

impl fmt::Display for Root {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.name(), self.entry.size().as_size_display())
    }
}

fn print_tree<T: DisplayableEntry>(entry: &T, options: &Options) {
    print_indented_tree(entry, options, 0);
}

fn print_indented_tree<T: DisplayableEntry>(entry: &T, options: &Options, level: usize) {
    println!("{0:1$}{2}", "", level * 2, entry);
    if options.depth_accepts(level) {
        let mut shown_entries = 0;

        for child in entry.children_iter() {
            if !options.should_show_hidden() && child.is_hidden() {
                continue;
            }

            print_indented_tree(child, options, level + 1);
            shown_entries += 1;

            if options.limit_reached(shown_entries) {
                break;
            }
        }
    }
}

fn main() {
    let options = arguments::parse();
    for root_path in options.roots() {
        match Root::for_path(root_path.clone()) {
            Ok(root) => print_tree(&root, &options),
            Err(message) =>
                println!("{}: {}", root_path.to_string_lossy(), message)
        }
    }
}
