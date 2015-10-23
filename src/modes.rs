use std::fmt;
use std::slice::Iter;

use arguments::Options;
use root::Root;
use entry::Entry;

pub trait DisplayableEntry : fmt::Display + Sized {
    type Child: DisplayableEntry;

    fn size(&self) -> u64;
    fn name(&self) -> &String;
    fn children_iter(&self) -> Iter<Self::Child>;
    fn is_file(&self) -> bool;

    fn is_hidden(&self) -> bool {
        self.name().chars().nth(0) == Some('.')
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Tree,
    Files,
}

impl Mode {
    pub fn work(&self, root: Root, options: &Options) {
        match self {
            &Mode::Tree => print_tree(root, options),
            &Mode::Files => print_largest_files(root, options),
        }
    }
}

fn print_tree<T: DisplayableEntry>(entry: T, options: &Options) {
    print_indented_tree(&entry, options, 0);
}

fn print_largest_files(root: Root, options: &Options) {
    if root.is_file() {
        // That was easy!
        println!("{}", root);
    } else {
        print_largest_files_in_directory(root, &options)
    }
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

fn print_largest_files_in_directory(root: Root, options: &Options) {
    let mut files = files_in(root.entry(), !options.should_show_hidden());
    let mut shown_files = 0;

    files.sort_by( |a, b| {
        // Note: We change the ordering to get in descending order
        b.size().cmp(&a.size())
    });

    println!("{}", root);
    for file in files {
        println!("  {}", file);

        shown_files += 1;
        if options.limit_reached(shown_files) {
            break;
        }
    }
}

fn files_in(entry: &Entry, skip_hidden: bool) -> Vec<&Entry> {
    if entry.is_file() {
        if entry.is_hidden() && skip_hidden {
            vec![]
        } else {
            vec![entry]
        }
    } else {
        entry.children_iter().flat_map(|child| files_in(child, skip_hidden)).collect()
    }
}
