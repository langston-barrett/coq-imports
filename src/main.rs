// --------------------------------------------------------------
// * coq-imports

extern crate coq_imports_lib;
use coq_imports_lib::{subcmd_organize, subcmd_insert, config_from_matches};

extern crate log;
extern crate env_logger;
extern crate pretty_env_logger;
use std::io::Write;
use log::LevelFilter;
use env_logger::{WriteStyle};
use pretty_env_logger::{formatted_builder};

extern crate clap;
use clap::{Arg, App, SubCommand};

// --------------------------------------------------------------
// ** Main

fn logging(verbosity: u8) {
    let mut builder = formatted_builder();
    (match verbosity {
        1 => builder.filter(None, LevelFilter::Error),
        2 => builder.filter(None, LevelFilter::Info),
        3 => builder.filter(None, LevelFilter::Debug),
        _ => builder.filter(None, LevelFilter::Warn),
    }).format(|buf, record| writeln!(buf, "{} - {}", record.level(), record.args()))
      .write_style(WriteStyle::Always).init();
}

fn main() -> std::io::Result<()> {

    let input = Arg::with_name("INPUT")
                .help("Input vernacular file")
                .required(true)
                .index(1);

    let matches = App::new("coq-imports")
                  .version("0.1")
                  .about("Organize your Coq imports")
                  .author("Langston B.")
                  .arg(Arg::with_name("v")
                      .short("v")
                      .multiple(true)
                      .help("Sets the level of verbosity"))
                  .subcommand(SubCommand::with_name("organize")
                              .about("organize imports")
                              .arg(input.clone()))
                  .subcommand(SubCommand::with_name("insert")
                              .about("insert an import")
                              .arg(input)
                              .arg(Arg::with_name("MODULE")
                                   .required(true)
                                   .index(2)))
                  .get_matches();

    logging(matches.occurrences_of("v") as u8);

    if let Some(sub_matches) = matches.subcommand_matches("organize") {
        subcmd_organize(config_from_matches(&matches, sub_matches))?;

    } else if let Some(sub_matches) = matches.subcommand_matches("insert") {
        subcmd_insert(config_from_matches(&matches, sub_matches),
                      sub_matches.value_of("MODULE").unwrap())?;
    }
    else {
        println!("Unrecognized subcommand (try `--help`)")
    }

    Ok(())
}
