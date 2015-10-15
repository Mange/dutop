use std::path::PathBuf;
use std::str::FromStr;
use std::process::exit;

enum Depth {
    Unlimited,
    Limited(usize)
}

impl Depth {
    fn accepts(&self, level: usize) -> bool {
        match *self {
            Depth::Unlimited => true,
            Depth::Limited(size) => size > level
        }
    }
}

impl FromStr for Depth {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        if s == "all" { return Ok(Depth::Unlimited); }

        let number = match s.parse::<usize>() {
            Ok(number) => number,
            Err(_) => return Err("Not a positive integer".to_string())
        };

        // usize is never negative. :-)
        if number == 0 {
            Ok(Depth::Unlimited)
        } else {
            Ok(Depth::Limited(number))
        }
    }
}

enum Limit {
    Unlimited,
    Limited(usize)
}

impl Limit {
    fn accepts(&self, index: usize) -> bool {
        match *self {
            Limit::Unlimited => true,
            Limit::Limited(count) => count > index
        }
    }
}

impl FromStr for Limit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        if s == "none" { return Ok(Limit::Unlimited); }

        let number = match s.parse::<usize>() {
            Ok(number) => number,
            Err(_) => return Err("Not a positive integer".to_string())
        };

        // usize is never negative. :-)
        if number == 0 {
            Ok(Limit::Unlimited)
        } else {
            Ok(Limit::Limited(number))
        }
    }
}

pub struct Options {
    roots: Vec<String>,
    limit: Limit,
    depth: Depth,
    show_all: bool,
}

impl Options {
    pub fn roots(&self) -> Vec<PathBuf> {
        self.roots.iter().map(|root| PathBuf::from(&root)).collect()
    }

    pub fn depth_accepts(&self, level: usize) -> bool {
        self.depth.accepts(level)
    }

    pub fn limit_accepts(&self, index: usize) -> bool {
        self.limit.accepts(index)
    }

    pub fn should_show_hidden(&self) -> bool {
        self.show_all
    }
}

pub fn parse() -> Options {
    let matches = clap_app!(dutop =>
        (version: "0.1")
        (author: "Magnus Bergmark <magnus.bergmark@gmail.com>")
        (about: "Prints the largest entries in a directory")
        (usage: "dutop [OPTIONS] [--] [DIR [DIR...]]")

        (@arg DIR: ... "The directories to look in (defaults to current working directory).")

        (@arg limit:
            -n [LIMIT]
            {|value| {
                if value == "none" { return Ok(()); }
                let parsed = value.parse::<usize>();
                if parsed.is_ok() {
                    Ok(())
                } else {
                    Err("Limit needs to be a non-negative integer or \"none\".".to_string())
                }
            }}
            "The max number of children shown per directory. Defaults to 1. 0 or \"none\" means \
                no limit."
        )

        (@arg depth:
            -d --depth [DEPTH]
            {|value| {
                if value == "all" { return Ok(()); }
                let parsed = value.parse::<usize>();
                if parsed.is_ok() {
                    Ok(())
                } else {
                    Err("Depth must be a non-negative integer or \"all\".".to_string())
                }
            }}
            "The depth to recurse when printing out entries. Defaults to 1. 0 or \"all\" means \
                unlimited depth."
        )

        (@arg recursive:
            -r --recursive
            "Show the entire tree instead of just the direct children. This implies \
                unlimited --depth."
        )

        (@arg all:
            -a --all
            "Show hidden files and directories. They are always counted for the total sum."
        )
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

    let limit = matches.value_of("limit").unwrap_or("1")
        .parse::<Limit>().unwrap_or_else(|error| {
            println!("Could not determine depth: {}", error);
            exit(2);
        });

    Options{
        roots: roots.iter().map(|value| value.to_string()).collect(),
        limit: limit,
        depth: depth,
        show_all: matches.is_present("all"),
    }
}
