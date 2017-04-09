use std::env::current_dir;
use std::error::Error;
use std::ffi::OsString;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use hyper::client::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use serde_json::from_reader;
use serde_json::Value as JsonValue;
use toml::Value as TomlValue;

pub type Result<T> = ::std::result::Result<T, Box<Error>>;

#[derive(Debug, Clone)]
pub struct MessageError(String);
impl MessageError {
    pub fn new<S: AsRef<str>>(msg: S) -> MessageError {
        MessageError(msg.as_ref().to_string())
    }
}
impl fmt::Display for MessageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Error for MessageError {
    fn description(&self) -> &str {
        self.0.as_ref()
    }
}

macro_rules! err {
  ($fmt:expr) => {
    Box::new(MessageError::new($fmt))
  };
  ($fmt:expr, $($arg:tt)*) => ({
    let msg = format!($fmt, $($arg)*);
    Box::new(MessageError(msg))
  });
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum UrlType {
    Documentation,
    Homepage,
    Repository,
    Crates,
}
impl UrlType {
    pub fn from_command<S: AsRef<str>>(s: S) -> Option<UrlType> {
        match s.as_ref() {
            "docs" | "documentation" => Some(UrlType::Documentation),
            "repo" | "repository" => Some(UrlType::Repository),
            "home" | "homepage" => Some(UrlType::Homepage),
            "crates" | "crate" | "cratesio" | "crates-io" | "crates.io" => Some(UrlType::Crates),
            _ => None,
        }
    }

    pub fn field(&self) -> &'static str {
        use UrlType::*;
        match *self {
            Documentation => "documentation",
            Homepage => "homepage",
            Repository => "repository",
            Crates => "", // N/A
        }
    }
}

fn get_url_from_toml(toml: &TomlValue, t: UrlType) -> Result<OsString> {
    if t == UrlType::Crates {
        let name = toml.get("package")
            .and_then(|v| v.get("name"))
            .ok_or_else(|| err!("Invalid Cargo.toml file"))?;
        Ok(OsString::from(format!("https://crates.io/crates/{}", name)))
    } else {
        match toml.get("package")
                  .and_then(|v| v.get(t.field()))
                  .and_then(|v| v.as_str()) {
            Some(url) => Ok(OsString::from(url)),
            None => {
                if t == UrlType::Documentation {
                    // usind docs.rs instead
                    let name = toml.get("package")
                        .and_then(|v| v.get("name"))
                        .ok_or_else(|| err!("Invalid Cargo.toml file"))?;
                    Ok(OsString::from(format!("https://docs.rs/{}", name)))
                } else {
                    Err(err!("{} URL is not registered in the crate", t.field()))
                }
            }
        }
    }
}

fn get_url_from_json(json: &JsonValue, t: UrlType) -> Result<OsString> {
    assert!(t != UrlType::Crates);
    match json.get("crate")
              .and_then(|v| v.get(t.field()))
              .and_then(|v| v.as_str()) {
        Some(url) => Ok(OsString::from(url)),
        None => Err(err!("{} URL is not registered in the crate", t.field())),
    }
}

pub fn get_url_of_this(t: UrlType) -> Result<OsString> {
    let cdir = current_dir()?;

    let mut cargo_toml_file = PathBuf::from(cdir);
    cargo_toml_file.push("Cargo.toml");

    if !cargo_toml_file.exists() {
        return Err(err!("Cargo.toml is missing"));
    }

    let cargo_desc: TomlValue = {
        let mut cargo_toml_content = String::new();
        File::open(cargo_toml_file)?
            .read_to_string(&mut cargo_toml_content)?;
        cargo_toml_content
            .parse::<TomlValue>()
            .map_err(|_| err!("Invalid Cargo.toml file"))?
    };

    get_url_from_toml(&cargo_desc, t)
}

pub fn get_url_of<S: AsRef<str>>(name: S, t: UrlType) -> Result<OsString> {
    if t == UrlType::Crates {
        Ok(OsString::from(format!("https://crates.io/crates/{}", name.as_ref())))
    } else {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let client = Client::with_connector(connector);
        let endpoint = format!("https://crates.io/api/v1/crates/{}", name.as_ref());
        let resp = client.get(endpoint.as_str()).send()?;
        let json = from_reader(resp)?;

        get_url_from_json(&json, t)
    }
}
