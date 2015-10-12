use clap::App;

pub struct Options {
    root: String,
}

impl Options {
    pub fn root(&self) -> &String {
        &self.root
    }
}

pub fn parse() -> Options {
    let matches = clap_app!(dutop =>
        (version: "0.1")
        (author: "Magnus Bergmark <magnus.bergmark@gmail.com>")
        (about: "Prints the largest entries in a directory")
        (@arg DIR: "The directory to look in")
    ).get_matches();

    Options{
        root: matches.value_of("DIR").unwrap_or(".").to_string(),
    }
}
