use std::path::PathBuf;
use std::str::FromStr;
use std::process::exit;

pub enum Depth {
    Unlimited,
    Limited(usize)
}

impl Depth {
    pub fn accepts(&self, level: usize) -> bool {
        match *self {
            Depth::Unlimited => true,
            Depth::Limited(size) => size > level
        }
    }
}

impl FromStr for Depth {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        let number = match s.parse::<usize>() {
            Ok(number) => number,
            Err(_) => return Err("Not a positive integer".to_string())
        };

        // number: usize is never negative. :-)
        if number == 0 {
            Ok(Depth::Unlimited)
        } else {
            Ok(Depth::Limited(number))
        }
    }
}

pub struct Options {
    roots: Vec<String>,
    depth: Depth,
}

impl Options {
    pub fn roots(&self) -> Vec<PathBuf> {
        self.roots.iter().map(|root| PathBuf::from(&root)).collect()
    }

    pub fn depth(&self) -> &Depth {
        &self.depth
    }
}

pub fn parse() -> Options {
    let matches = clap_app!(dutop =>
        (version: "0.1")
        (author: "Magnus Bergmark <magnus.bergmark@gmail.com>")
        (about: "Prints the largest entries in a directory")

        (@arg DIR: ... "The directories to look in (defaults to current working directory).")

        (@arg depth:
            -d --depth [INT]
            {|value| {
                let parsed = value.parse::<usize>();
                if parsed.is_ok() {
                    Ok(())
                } else {
                    Err("Not a positive integer".to_string())
                }
            }}
            "The depth to recurse when printing out entries. Defaults to 1. 0 means unlimited depth."
        )

        (@arg recursive: -r --recursive "Show the entire tree instead of just the direct children. This implies unlimited --depth.")
    ).get_matches();

    let roots = matches.values_of("DIR").unwrap_or(vec!["."]);

    let default_depth = if matches.is_present("recursive") {
        Depth::Unlimited
    } else {
        Depth::Limited(1)
    };

    let depth = match matches.is_present("depth") {
        true => {
            // We can unwrap since the argument is required. We'd never get here unless the value
            // exists.
            let value = matches.value_of("depth").unwrap();
            value.parse::<Depth>().unwrap_or_else(|error| {
                println!("Could not determine depth: {}", error);
                exit(2);
            })
        },
        false => default_depth
    };

    Options{
        roots: roots.iter().map(|value| value.to_string()).collect(),
        depth: depth,
    }
}
