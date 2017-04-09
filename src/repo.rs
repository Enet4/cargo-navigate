//! #![deny(missing_docs,
//!        missing_debug_implementations,
//!        missing_copy_implementations,
//!        trivial_casts, trivial_numeric_casts,
//!        unsafe_code,
//!        unused_import_braces,
//!        unused_qualifications)]

extern crate clap;
extern crate cargo;
extern crate hyper;
extern crate hyper_native_tls;
extern crate open;
extern crate toml;
extern crate serde_json;

mod urls;

use std::io::{stderr, Write};
use clap::{App, Arg, ArgMatches};
use urls::{UrlType, Result, get_url_of, get_url_of_this};

fn main() {
    let args = App::new("cargo repo")
        .about("Navigate to a crate's repository")
        .bin_name("cargo")
        .arg(Arg::with_name("crate")
            .required(false)
            .help("The crate to navigate to (or the crate in the current working directory if \
                   unspecified)"))
        .get_matches();

    run(&args)
        .err()
        .map(|e| {
            write!(stderr(), "{}\n", e.to_string()).unwrap();
            std::process::exit(101);
        });
}

fn run(args: &ArgMatches) -> Result<()> {
    let t = UrlType::Repository;
    let url = match args.value_of("crate") {
        Some(c) => get_url_of(c, t),
        None => get_url_of_this(t),
    }?;

    open::that(url)?;
    Ok(())
}
