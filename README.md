# dutop

[![Build Status](https://travis-ci.org/Mange/dutop.svg?branch=master)](https://travis-ci.org/Mange/dutop) [![license](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/Mange/dutop/blob/master/LICENSE) [![Coverage Status](https://coveralls.io/repos/Mange/dutop/badge.svg?branch=master&service=github)](https://coveralls.io/github/Mange/dutop?branch=master)


Simple command to list the largest files and/or directories recursively.

## Usage

```
USAGE:
	dutop [OPTIONS] [--] [DIR [DIR...]]

FLAGS:
    -a, --all          Show hidden files and directories. They are always counted for the total sum.
        --files        Print the largest files instead of a tree. Depth will say how far down to look for the "largest" file.
    -h, --help         Prints help information
    -r, --recursive    Show the entire tree instead of just the direct children. This implies unlimited --depth.
    -V, --version      Prints version information

OPTIONS:
    -d, --depth <DEPTH>    The depth to recurse when printing out entries. Defaults to 1. 0 or "all" means unlimited depth.
    -n <LIMIT>             The max number of children shown per directory. Defaults to 1. 0 or "all" means no limit.

ARGS:
    DIR...    The directories to look in (defaults to current working directory).
```

## Installation

Download the source code and Rust.

```
cargo build --release
cp target/release/dutop ~/bin/dutop
```

## License

MIT

Copyright © 2015 Magnus Bergmark
