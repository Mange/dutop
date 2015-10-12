use std::path::PathBuf;

pub struct Options {
    root: String,
}

impl Options {
    pub fn root(&self) -> PathBuf {
        PathBuf::from(&self.root)
    }
}

pub fn parse() -> Options {
    let matches = clap_app!(dutop =>
        (version: "0.1")
        (author: "Magnus Bergmark <magnus.bergmark@gmail.com>")
        (about: "Prints the largest entries in a directory")
        (@arg DIR: "The directory to look in (defaults to current working directory)")
    ).get_matches();

    Options{
        root: matches.value_of("DIR").unwrap_or(".").to_string(),
    }
}
