#[macro_use]
extern crate clap;

mod arguments;
mod utils;
mod entry;
mod root;
mod modes;

use root::Root;

fn main() {
    let options = arguments::parse();
    for root_path in options.roots() {
        match Root::for_path(&root_path) {
            Ok(root) => options.mode().work(root, &options),
            Err(message) =>
                println!("{}: {}", root_path.to_string_lossy(), message)
        }
    }
}
