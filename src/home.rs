//! #![deny(missing_docs,
//!        missing_debug_implementations,
//!        missing_copy_implementations,
//!        trivial_casts, trivial_numeric_casts,
//!        unsafe_code,
//!        unused_import_braces,
//!        unused_qualifications)]

extern crate clap;
extern crate hyper;
extern crate hyper_native_tls;
extern crate open;
extern crate toml;
extern crate serde_json;

mod urls;

use std::io::{stderr, Write};
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use urls::{UrlType, Result, get_url_of, get_url_of_this};

fn main() {
    let args = App::new("cargo home")
        .about("Navigate to a crate's homepage")
        .bin_name("cargo")
        .subcommand(SubCommand::with_name("home")
            .about("Navigate to a crate's homepage")
            .arg(Arg::with_name("crate")
                .required(false)
                .help("The crate to navigate to (or the crate in the current working directory if \
                    unspecified)")))
            .settings(&[AppSettings::SubcommandRequired])
        .get_matches();

    run(args.subcommand_matches("home").unwrap())
        .err()
        .map(|e| {
            write!(stderr(), "{}\n", e.to_string()).unwrap();
            std::process::exit(101);
        });
}

fn run(args: &ArgMatches) -> Result<()> {
    let t = UrlType::Homepage;
    let url = match args.value_of("crate") {
        Some(c) => get_url_of(c, t),
        None => get_url_of_this(t),
    }?;

    open::that(url)?;
    Ok(())
}
