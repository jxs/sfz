// Copyright (c) 2018 Weihang Lo
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::env;
use std::path::PathBuf;
use std::net::SocketAddr;
use std::fs::canonicalize;

use clap::App;

use BoxResult;

#[derive(Debug, Clone)]
pub struct Args {
    pub address: String,
    pub port: u16,
    pub cache: u32,
    pub cors: bool,
    pub compress: bool,
    pub path: PathBuf,
    pub all: bool,
    pub ignore: bool,
    pub follow_links: bool,
}

impl Args {
    /// Parse command-line arguments.
    ///
    /// If a parsing error ocurred, exit the process and print out informative
    /// error message to user.
    pub fn parse(app: App) -> BoxResult<Args> {
        let matches = app.get_matches();
        let address = matches.value_of("address")
            .unwrap_or_default()
            .to_owned();
        let port = value_t!(matches.value_of("port"), u16)?;
        let cache = value_t!(matches.value_of("cache"), u32)?;
        let cors = matches.is_present("cors");
        let path = matches.value_of("path").unwrap_or_default();
        let path = Args::parse_path(path)?;

        let compress = !matches.is_present("unzipped");
        let all = matches.is_present("all");
        let ignore = !matches.is_present("no-ignore");
        let follow_links = matches.is_present("follow-links");

        Ok(Args {
            address,
            port,
            cache,
            cors,
            path,
            compress,
            all,
            ignore,
            follow_links,
        })
    }

    /// Parse path.
    fn parse_path(path: &str) -> BoxResult<PathBuf> {
        let path = PathBuf::from(path);
        if !path.exists() {
            bail!("error: path \"{}\" doesn't exist", path.display());
        }
        if path.is_absolute() {
            Ok(path)
        } else {
            env::current_dir()
                .map(|p| p.join(&path))
                .and_then(canonicalize)
                .or_else(|err| bail!(
                    "error: failed to access path \"{}\": {}",
                    path.display(),
                    err,
                ))
        }
    }

    /// Construct socket address from arguments.
    pub fn address(&self) -> BoxResult<SocketAddr> {
        format!("{}:{}", self.address, self.port).parse()
            .or_else(|err| bail!(
                "error: invalid address {}:{} : {}",
                self.address,
                self.port,
                err,
            ))
    }
}
