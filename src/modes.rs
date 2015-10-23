use std::fmt;
use std::slice::Iter;

use arguments::Options;
use root::Root;

pub trait DisplayableEntry : fmt::Display + Sized {
    type Child: DisplayableEntry;

    fn size(&self) -> u64;
    fn name(&self) -> &String;
    fn children_iter(&self) -> Iter<Self::Child>;

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
            &Mode::Files => panic!("Not yet implemented"),
        }
    }
}

fn print_tree<T: DisplayableEntry>(entry: T, options: &Options) {
    print_indented_tree(&entry, options, 0);
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
