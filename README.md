# Grep

A tutorial exercise - a "grep"-like program written in Rust.

## How to run

Type the following command in repository root directory:

```
# Powershell:
cat input.txt | & cargo run -- <ARGUMENTS>

$ Bash:
cat input.txt | cargo run -- <ARGUMENTS>
```

## Command line arguments

* `-i`, `--ignore-case`- turns "ignore case" mode on.
* `-r`, `--regex` - turns "regular expression" mode on.
* `-v`, `--verbose` - enables verbose output.
* `-?`, `-h`, `--help`- prints help and exits.

All other arguments are treated as search patterns.

## Usage

`rustgrep` scans standard input line per line. Once a line matches at least one of patterns it's printed to standard output.

`rustgrep` doesn't support reading input from a file.