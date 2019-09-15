extern crate clap;
use clap::{Arg, App, SubCommand};
use std::env;
use std::process;

fn main() {
    let cli = App::new(env::var("CARGO_PKG_NAME").unwrap().as_str())
        .version(env::var("CARGO_PKG_VERSION").unwrap().as_str())
        .author(env::var("CARGO_PKG_AUTHORS").unwrap().as_str())
        .about(env::var("CARGO_PKG_DESCRIPTION").unwrap().as_str())
        .subcommand(SubCommand::with_name("get")
                    .arg(Arg::with_name("key")))
        .subcommand(SubCommand::with_name("set")
                    .arg(Arg::with_name("key"))
                    .arg(Arg::with_name("value")))
        .subcommand(SubCommand::with_name("rm")
                    .arg(Arg::with_name("key")))
        .get_matches();

    match cli.subcommand_name() {
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

