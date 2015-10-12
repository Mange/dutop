use std::path::PathBuf;

pub struct Options {
    roots: Vec<String>,
}

impl Options {
    pub fn roots(&self) -> Vec<PathBuf> {
        self.roots.iter().map(|root| PathBuf::from(&root)).collect()
    }
}

pub fn parse() -> Options {
    let matches = clap_app!(dutop =>
        (version: "0.1")
        (author: "Magnus Bergmark <magnus.bergmark@gmail.com>")
        (about: "Prints the largest entries in a directory")
        (@arg DIR: ... "The directories to look in (defaults to current working directory)")
    ).get_matches();

    let roots = matches.values_of("DIR").unwrap_or(vec!["."]);
    Options{
        roots: roots.iter().map(|value| value.to_string()).collect(),
    }
}
