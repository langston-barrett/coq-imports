// --------------------------------------------------------------
// * coq-imports

extern crate clap;

use std::fs::File;
use std::io::{BufReader};
use std::io::prelude::*;
use std::iter::FromIterator;

// --------------------------------------------------------------
// ** Config

pub struct Config<'a> {
    target: &'a str,
    verbosity: u8,
}

pub fn config_from_matches<'a>(matches: &'a clap::ArgMatches) -> Config<'a> {
    Config {
        target: matches.value_of("INPUT").unwrap(),
        verbosity: matches.occurrences_of("v") as u8,
    }
}

// --------------------------------------------------------------
// ** Subcommands


// --------------------------------------------------------------
// *** Organize

pub fn subcmd_organize(conf: Config) -> std::io::Result<()> {
    Ok(())
}

// --------------------------------------------------------------
// *** Insert

pub fn package_components(line: &str) -> Vec<String> {
    let words = Vec::from_iter(line.split_whitespace());
    assert!(words.len() == 3); // Require Import Stuff.
    Vec::from_iter(words[2].split(".").map(String::from))
}

pub fn matching_prefix<T : PartialEq> (v1: Vec<&T>, v2: Vec<&T>) -> u32 {
    let mut m = 0;
    for (x1, x2) in v1.iter().zip(v2.iter()) {
        if x1 == x2 {
            m = m + 1;
        }
    }
    m
}

// Takes a function in the State monad (with state T), and repeatedly applies it
// to the lines of the file, yielding an iterator over the filtered lines.
pub fn filter_lines<T, F>(filename: &str, initial_state: &mut T, f: F)
    where F : Fn(&mut T, &str) -> bool
{
    let file = File::open(conf.target)?;
    let file = BufReader::new(file);
    file.iter().fold()
}

// For each line, if the line starts with "Require Import",
//  - Get the number of matching module components
//  - If this is bigger than the current max, it is the new max
//    + Record this line and its number
//    + If this line is lexicographically smaller than the module we're inserting,
//      set "before" to false. Otherwise, set it to true.
//  - If this is equal to the current max,
//    + If "before" is false and this line is lexicographically smaller than
//      the module we're inserting, record this line and its number.

pub fn subcmd_insert(conf: Config, module: &str) -> std::io::Result<()> {
    let file = File::open(conf.target)?;
    let file = BufReader::new(file);

    let components             = package_components(module);
    let mut before             = true; // insert before max_matching_lino?
    let mut max_matching       = 0;
    let mut max_matching_lino  = 0;
    let mut max_matching_line  = vec![];
    // let mut current_line       = String::new();
    for (_lino, line) in file.lines().enumerate() {
        if let Ok(l) = line {
            let mut current_components = package_components(&l);
            if l.starts_with("Require Import") {
                let matching = matching_prefix(vec![&"x"], vec![&"y"]);
                if matching > max_matching {
                    // Yay - we've found a line that matches more!
                    max_matching      = matching;
                    max_matching_line = current_components;

                    if max_matching_line <= components {
                        before = false;
                    } else {
                        before = true;
                    }
                } else if matching == max_matching && ! before {
                    // It matches just as much. Perhaps our new import should
                    // be inserted after this one instead.
                }
            }

        }
    }

    Ok(())
    // file.write_all(b"Hello, world!")?;
}

// --------------------------------------------------------------

// ** Tests

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
