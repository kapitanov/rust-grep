use std::error::Error;

extern crate regex;
use regex::Regex;
use regex::RegexBuilder;

// ----------------------------------------------------------------------------
// Runner configration
// ----------------------------------------------------------------------------

trait Pattern {
    fn is_match(&self, s: &str) -> bool;
}

trait PatternFactory {
    fn name(&self) -> String;
    fn try_create<'a>(&self, s: &str, ignore_case: bool) -> Result<Box<Pattern>, String>;
}

pub struct Config {
    patterns: Vec<Box<Pattern>>,
    show_help: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 1 {
            return Err(String::from("not enough arguments"));
        }

        let mut patterns: Vec<Box<Pattern>> = vec![];
        let mut ignore_case = false;
        let mut show_help = false;
        let mut verbose = false;

        let mut factory = TextPatternFactory::instance();

        let mut pattern_strs: Vec<&String> = vec![];
        for arg in args {
            match arg.as_str() {
                "-i" | "--ignore-case" => {
                    ignore_case = true;
                }
                "-r" | "--regex" | "--regexp" => {
                    factory = RegexPatternFactory::instance();
                }
                "-v" | "--verbose" => {
                    verbose = true;
                }
                "-?" | "-h" | "--help" => {
                    show_help = true;
                }
                _ => {
                    pattern_strs.push(arg);
                }
            }
        }

        if !show_help {
            if verbose {
                eprintln!("* verbose: {}", true);
                eprintln!("* ignore_case: {}", ignore_case);
                eprintln!("* pattern_type: {}", factory.name());
            }

            if verbose {
                eprintln!("* patterns: [");
            }

            for pattern_str in pattern_strs {
                if verbose {
                    eprintln!("*   \"{}\",", pattern_str);
                }
                match factory.try_create(pattern_str, ignore_case) {
                    Ok(pattern) => patterns.push(pattern),
                    Err(e) => {
                        return Err(e);
                    }
                }
            }

            if verbose {
                eprintln!("* ]");
            }
        }

        Ok(Config {
            patterns,
            show_help,
        })
    }

    pub fn show_help(&self) -> bool {
        self.show_help
    }

    pub fn create_runner(&self) -> Runner {
        Runner::new(self)
    }
}

// ----------------------------------------------------------------------------
// Text pattern
// ----------------------------------------------------------------------------

struct TextPattern {
    q: String,
}

impl Pattern for TextPattern {
    fn is_match(&self, s: &str) -> bool {
        s.contains(self.q.as_str())
    }
}

struct IgnoreCaseTextPattern {
    q: String,
}

impl Pattern for IgnoreCaseTextPattern {
    fn is_match(&self, s: &str) -> bool {
        s.to_lowercase().contains(self.q.as_str())
    }
}

struct TextPatternFactory;

impl TextPatternFactory {
    fn instance<'a>() -> &'a PatternFactory {
        &TextPatternFactory {}
    }
}

impl PatternFactory for TextPatternFactory {
    fn name(&self) -> String {
        String::from("text")
    }

    fn try_create<'a>(&self, s: &str, ignore_case: bool) -> Result<Box<Pattern>, String> {
        if ignore_case {
            Ok(Box::new(IgnoreCaseTextPattern {
                q: String::from(s.to_lowercase()),
            }))
        } else {
            Ok(Box::new(TextPattern { q: String::from(s) }))
        }
    }
}

// ----------------------------------------------------------------------------
// Regexp pattern
// ----------------------------------------------------------------------------

struct RegexPattern {
    regex: Regex,
}

impl Pattern for RegexPattern {
    fn is_match(&self, s: &str) -> bool {
        self.regex.is_match(s)
    }
}

struct RegexPatternFactory;

impl RegexPatternFactory {
    fn instance<'a>() -> &'a PatternFactory {
        &RegexPatternFactory {}
    }
}

impl PatternFactory for RegexPatternFactory {
    fn name(&self) -> String {
        String::from("regex")
    }

    fn try_create<'a>(&self, s: &str, ignore_case: bool) -> Result<Box<Pattern>, String> {
        let mut builder = RegexBuilder::new(s);
        builder.case_insensitive(ignore_case);

        match builder.build() {
            Ok(regex) => Ok(Box::new(RegexPattern { regex })),
            Err(e) => {
                let err = String::from(e.description());
                return Err(err);
            }
        }
    }
}

// ----------------------------------------------------------------------------
// Runner
// ----------------------------------------------------------------------------

pub struct Runner<'a> {
    config: &'a Config,
}

impl<'a> Runner<'a> {
    fn new(config: &'a Config) -> Runner<'a> {
        Runner { config: config }
    }

    pub fn is_match(&self, s: &str) -> bool {
        for pattern in &self.config.patterns {
            if pattern.is_match(s) {
                return true;
            }
        }

        false
    }
}

// ----------------------------------------------------------------------------
// Tests
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_none() {
        let args = [String::from("foo"), String::from("bar")];
        let config = Config::new(&args).unwrap();
        let runner = config.create_runner();

        assert_eq!(false, runner.is_match("bat"));
    }

    #[test]
    fn contains_one() {
        let args = [String::from("foo"), String::from("bar")];
        let config = Config::new(&args).unwrap();
        let runner = config.create_runner();

        assert_eq!(true, runner.is_match("xbarbat"));
    }

    #[test]
    fn contains_few() {
        let args = [String::from("foo"), String::from("bar")];
        let config = Config::new(&args).unwrap();
        let runner = config.create_runner();

        assert_eq!(true, runner.is_match("xbarbatFoox"));
    }
}
