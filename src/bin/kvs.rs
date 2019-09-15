extern crate clap;
use clap::{Arg, App, SubCommand};
use std::process;

fn main() {
    let matches = App::new("kvs")
        .version("0.1.0")
        .subcommand(SubCommand::with_name("get")
                    .arg(Arg::with_name("key")))
        .subcommand(SubCommand::with_name("set")
                    .arg(Arg::with_name("key"))
                    .arg(Arg::with_name("value")))
        .subcommand(SubCommand::with_name("rm")
                    .arg(Arg::with_name("key")))
        .get_matches();

    match matches.subcommand_name() {
        Some("get") => {
            eprintln!("unimplemented");
            process::exit(1);
        },
        Some("set") => {
            eprintln!("unimplemented");
            process::exit(1);
        },
        Some("rm") => {
            eprintln!("unimplemented");
            process::exit(1);
        },
        _ => { process::exit(1) },
    }
}

