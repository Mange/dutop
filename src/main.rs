#[macro_use]
extern crate clap;

use std::fmt;
use std::slice::Iter;

mod arguments;
mod utils;
mod entry;
mod root;

use root::Root;

use arguments::Options;

trait DisplayableEntry : fmt::Display + Sized {
    type Child: DisplayableEntry;

    fn size(&self) -> u64;
    fn name(&self) -> &String;
    fn children_iter(&self) -> Iter<Self::Child>;

    fn is_hidden(&self) -> bool {
        self.name().chars().nth(0) == Some('.')
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
        match Root::for_path(&root_path) {
            Ok(root) => print_tree(&root, &options),
            Err(message) =>
                println!("{}: {}", root_path.to_string_lossy(), message)
        }
    }
}
