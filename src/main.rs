#[macro_use]
extern crate clap;

mod arguments;
mod utils;
mod entry;
mod root;
mod modes;

use root::Root;

use arguments::Options;

use modes::Mode;

fn work(root: Root, options: &Options) {
    match options.mode() {
        &Mode::Tree => modes::print_tree(root, options),
        &Mode::Files => panic!("Not yet implemented")
    }
}

fn main() {
    let options = arguments::parse();
    for root_path in options.roots() {
        match Root::for_path(&root_path) {
            Ok(root) => work(root, &options),
            Err(message) =>
                println!("{}: {}", root_path.to_string_lossy(), message)
        }
    }
}
