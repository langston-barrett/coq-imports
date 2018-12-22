// --------------------------------------------------------------
// * coq-imports

extern crate coq_imports_lib;
use coq_imports_lib::{subcmd_organize, subcmd_insert, config_from_matches};

extern crate clap;
use clap::{Arg, App, SubCommand};

// --------------------------------------------------------------
// ** Main

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

    if let Some(matches) = matches.subcommand_matches("organize") {
        subcmd_organize(config_from_matches(matches))?;

    } else if let Some(matches) = matches.subcommand_matches("insert") {
        subcmd_insert(config_from_matches(matches),
                      matches.value_of("INPUT").unwrap())?;
    }
    else {
        println!("Unrecognized subcommand (try `--help`)")
    }

    Ok(())
}
