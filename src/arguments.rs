use std::env;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;

use modes::Mode;

#[derive(Debug, PartialEq, Eq)]
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
            Err(_) => return Err("Not a positive integer or \"all\"".to_string())
        };

        if number > 0 {
            Ok(Depth::Limited(number))
        } else {
            Ok(Depth::Unlimited)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Limit {
    Unlimited,
    Limited(usize)
}

impl FromStr for Limit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        if s == "all" { return Ok(Limit::Unlimited); }

        let number = match s.parse::<usize>() {
            Ok(number) => number,
            Err(_) => return Err("Not a positive integer or \"all\"".to_string())
        };

        if number > 0 {
            Ok(Limit::Limited(number))
        } else {
            Ok(Limit::Unlimited)
        }
    }
}

pub struct Options {
    roots: Vec<String>,
    limit: Limit,
    depth: Depth,
    mode: Mode,
    show_all: bool,
}

impl Options {
    pub fn roots(&self) -> Vec<PathBuf> {
        self.roots.iter().map(|root| PathBuf::from(&root)).collect()
    }

    pub fn mode(&self) -> &Mode {
        &self.mode
    }

    pub fn depth_accepts(&self, level: usize) -> bool {
        self.depth.accepts(level)
    }

    pub fn limit_reached(&self, shown_entries: usize) -> bool {
        match self.limit {
            Limit::Unlimited => false,
            Limit::Limited(max) => max <= shown_entries
        }
    }

    pub fn should_show_hidden(&self) -> bool {
        self.show_all
    }
}

pub fn parse() -> Options {
    parse_from(env::args())
}

// Provided for tests
fn parse_from<I, T>(iterator: I) -> Options
    where I: IntoIterator<Item = T>,
          T: AsRef<OsStr> {
    let matches = clap_app!(dutop =>
        (version: "0.1")
        (author: "Magnus Bergmark <magnus.bergmark@gmail.com>")
        (about: "Prints the largest entries in a directory")
        (usage: "dutop [OPTIONS] [--] [DIR [DIR...]]")

        (@arg DIR: ... "The directories to look in (defaults to current working directory).")

        (@arg limit:
            -n [LIMIT]
            {|value| {
                if value == "all" { return Ok(()); }
                let parsed = value.parse::<usize>();
                if parsed.is_ok() {
                    Ok(())
                } else {
                    Err("Limit needs to be a non-negative integer or \"all\".".to_string())
                }
            }}
            "The max number of children shown per directory. Defaults to 1. 0 or \"all\" means \
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

        (@arg files:
            --files
            "Print the largest files instead of a tree. Depth will say how far down to look for \
                the \"largest\" file."
        )
    ).get_matches_from(iterator);

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

    let mode = match matches.is_present("files") {
        true => Mode::Files,
        false => Mode::Tree,
    };

    Options{
        roots: roots.iter().map(|value| value.to_string()).collect(),
        limit: limit,
        depth: depth,
        mode: mode,
        show_all: matches.is_present("all"),
    }
}

#[cfg(test)]
mod tests {
    use super::{Depth,Limit,parse_from};
    use std::path::PathBuf;
    use modes::Mode;

    // parse_from and Option

    #[test]
    fn it_has_defaults_on_no_arguments() {
        let options = parse_from(vec!["dutop"]);

        assert_eq!(options.roots, vec!["."]);
        assert_eq!(options.limit, Limit::Limited(1));
        assert_eq!(options.depth, Depth::Limited(1));
        assert_eq!(options.should_show_hidden(), false);

        assert_eq!(options.limit_reached(0), false);
        assert_eq!(options.limit_reached(1), true);

        assert_eq!(options.depth_accepts(0), true);
        assert_eq!(options.depth_accepts(1), false);
    }

    #[test]
    fn it_takes_multiple_roots() {
        let options = parse_from(vec!["dutop", "foo", "bar"]);
        assert_eq!(options.roots, vec!["foo", "bar"]);
        assert_eq!(options.roots(), vec![PathBuf::from("foo"), PathBuf::from("bar")]);
    }

    #[test]
    fn options_has_depth_information() {
        let options = parse_from(vec!["dutop", "-d", "0"]);
        assert_eq!(options.depth, Depth::Unlimited);
    }

    #[test]
    fn options_has_limit_information() {
        let options = parse_from(vec!["dutop", "-n", "0"]);
        assert_eq!(options.limit, Limit::Unlimited);
    }

    #[test]
    fn options_has_show_all_option() {
        let options = parse_from(vec!["dutop", "-a"]);
        assert_eq!(options.should_show_hidden(), true);
    }

    #[test]
    fn options_defaults_to_unlimited_depth_when_recursive() {
        let options = parse_from(vec!["dutop", "-r"]);
        assert_eq!(options.depth, Depth::Unlimited);
    }

    #[test]
    fn options_can_override_depth_when_recursive() {
        let options = parse_from(vec!["dutop", "-d", "5", "-r"]);
        assert_eq!(options.depth, Depth::Limited(5));
    }

    #[test]
    fn options_default_to_tree_mode() {
        let options = parse_from(vec!["dutop"]);
        assert_eq!(options.mode(), &Mode::Tree);
    }

    #[test]
    fn options_can_select_file_mode() {
        let options = parse_from(vec!["dutop", "--files"]);
        assert_eq!(options.mode(), &Mode::Files);
    }

    // Depth

    #[test]
    fn it_parses_positive_depth_from_strings() {
        assert_eq!("12".parse::<Depth>(), Ok(Depth::Limited(12)));
    }

    #[test]
    fn it_parses_zero_depth_from_strings() {
        assert_eq!("0".parse::<Depth>(), Ok(Depth::Unlimited));
    }

    #[test]
    fn it_parses_named_alias_for_unlimited_depth_from_strings() {
        assert_eq!("all".parse::<Depth>(), Ok(Depth::Unlimited));
    }

    #[test]
    fn it_rejects_broken_depth_strings() {
        assert_eq!(
            "totally broken".parse::<Depth>(),
            Err("Not a positive integer or \"all\"".to_string())
        );
    }

    // Limit

    #[test]
    fn it_parses_positive_limit_from_strings() {
        assert_eq!("12".parse::<Limit>(), Ok(Limit::Limited(12)));
    }

    #[test]
    fn it_parses_zero_limit_from_strings() {
        assert_eq!("0".parse::<Limit>(), Ok(Limit::Unlimited));
    }

    #[test]
    fn it_parses_named_alias_for_unlimited_limit_from_strings() {
        assert_eq!("all".parse::<Limit>(), Ok(Limit::Unlimited));
    }

    #[test]
    fn it_rejects_broken_limit_strings() {
        assert_eq!(
            "totally broken".parse::<Limit>(),
            Err("Not a positive integer or \"all\"".to_string())
        );
    }
}
