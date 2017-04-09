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

#[macro_use]
mod urls;
use urls::{Result, UrlType, get_url_of, get_url_of_this};

use std::io::{stderr, Write};
use cargo::Config;
use clap::{App, Arg, ArgMatches};

fn main() {
    let args = App::new("cargo navigate")
        .about("Navigate to a crate's informative link: homepage, crates.io, repository or documentation")
        .bin_name("cargo")
        .arg(Arg::with_name("where")
            .required(true)
            .possible_values(&["repo", "home", "docs", "crates", "crate", "crates.io", "crates-io",
                "repository", "homepage", "documentation", "cratesio", "crates-io"])
            .hide_possible_values(true)
            .default_value("repo")
            .help("Where to navigate (\"repo\", \"docs\", \"crates\", \"home\")"))
        .arg(Arg::with_name("crate")
            .required(false)
            .help("The crate to navigate to (or the crate in the current working directory if \
                   unspecified)"))
        .get_matches();

    let config = Config::default().expect("Unable to get config");
    run(&config, &args)
        .err()
        .map(|e| {
                 write!(stderr(), "{}\n", e.to_string()).unwrap();
                 std::process::exit(101);
             });
}

fn run(_: &Config, args: &ArgMatches) -> Result<()> {
    let t = UrlType::from_command(args.value_of("where").unwrap()).unwrap();

    let url = match args.value_of("crate") {
        Some(c) => get_url_of(c, t),
        None => get_url_of_this(t),
    }?;

    open::that(url)?;
    Ok(())
}
