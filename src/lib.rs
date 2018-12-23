// --------------------------------------------------------------
// * coq-imports

#[macro_use]
extern crate log;
extern crate clap;

use std::fs::File;
use std::io::{BufReader};
use std::io::prelude::*;
use std::iter::FromIterator;

// --------------------------------------------------------------
// ** Config

pub struct Config<'a> {
    pub target: &'a str,
}

pub fn config_from_matches<'a>(matches: &'a clap::ArgMatches,
                               sub_matches: &'a clap::ArgMatches) -> Config<'a> {
    Config {
        target: sub_matches.value_of("INPUT").unwrap(),
    }
}

// --------------------------------------------------------------
// ** Subcommands


// --------------------------------------------------------------
// *** Organize

pub fn subcmd_organize(_conf: Config) -> std::io::Result<()> {
    Ok(())
}

// --------------------------------------------------------------
// *** Insert

// Drop the keywords to import the module
pub fn drop_import(line: &str) -> &str {
    Vec::from_iter(line.split_whitespace())[2]
}

pub fn package_components(line: &mut str) -> Vec<String> {
    Vec::from_iter(line.split(".").map(String::from))
}

pub fn matching_prefix<T : PartialEq> (v1: &Vec<T>, v2: &Vec<T>) -> u32 {
    let mut m = 0;
    for (x1, x2) in v1.iter().zip(v2.iter()) {
        if x1 == x2 {
            m = m + 1;
        }
    }
    m
}

// Out of a list of imports and their line numbers, find the one
// which matches "module" most closely and return its number.
pub fn max_matching_line<'a, I>(module: &'a str, file: I) -> usize
    where I : Iterator<Item = (usize, String)>
{
    let components             = package_components(&mut String::from(module)[..]);
    let mut before             = true; // insert before max_matching_lino?
    let mut max_matching       = 0;
    let mut max_matching_lino  = 0;
    let mut max_matching_line  = vec![];
    // let mut current_line       = String::new();
    for (lino, line) in file {
        let current_components =
            package_components(&mut String::from(drop_import(&line[..])));
        let matching = matching_prefix(&current_components, &components);
        if matching > max_matching {
            // Yay - we've found a line that matches more!
            max_matching      = matching;
            max_matching_lino = lino;
            max_matching_line = current_components;

            if max_matching_line <= components {
                before = false;
            } else {
                before = true;
            }
        } else if matching == max_matching && ! before {
            // It matches just as much. Perhaps our new import should
            // be inserted after this one instead.
            if current_components <= components &&
                current_components >= max_matching_line {
                max_matching      = matching;
                max_matching_lino = lino;
                max_matching_line = current_components;
            }
        }
    }
    max_matching_lino
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
    debug!("Inserting module {}", module);

    let read = File::open(conf.target)?;
    let read = BufReader::new(read);
    debug!("Opened file for reading: {}", conf.target);

    // Get the line numbers and content of only the import lines
    let read = read.lines().enumerate().filter_map(|(lino, line)| {
        if let Ok(ln) = line {
            if ln.starts_with("Require Import") {
                debug!("Found import line {}", ln);
                return Some((lino, ln))
            }
        }
        debug!("Returning None");
        None
    });

    let lino      = max_matching_line(module, read);

    let mut readf = File::open(conf.target)?;
    let mut read  = String::new();
    readf.read_to_string(&mut read)?;

    let mut write = File::create(conf.target)?;
    debug!("Opened for writing {}", conf.target);
    let to_insert = format!("Require Import {}.", module);
    let mut to_write = Vec::from_iter(read.lines());
    to_write.insert(lino + 1, &to_insert[..]);
    write.write_all(to_write.join("\n").as_bytes())
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
