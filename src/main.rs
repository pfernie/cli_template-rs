#[deny(warnings)]
use std::result::Result as StdResult;
use std::{env, fs};

use anyhow::anyhow;
use rson_rs as rson;
use serde_derive::Deserialize;
use structopt::StructOpt;
use tracing::debug;
use tracing_subscriber;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(
        short = "c",
        long = "config-file",
        default_value = "{{project-name}}.rson"
    )]
    /// Configuration file to use.
    config_file: String,
    #[structopt(short = "v")]
    /// If specified, debug output will be enabled.
    verbose: bool,
}

#[derive(Debug, Deserialize)]
struct Config {}

impl Config {
    pub fn load(path: &str) -> Result<Config> {
        let config_file = fs::read_to_string(path)
            .map_err(|e| anyhow!("unable to read configuration file: {}", e))?;
        rson::de::from_str(&config_file)
            .map_err(|e| anyhow!("invalid config file {}: {}", path, e))
            .and_then(Config::validate)
    }

    fn validate(config: Config) -> Result<Config> {
        Ok(config)
    }
}

type Error = anyhow::Error;
type Result<T> = StdResult<T, Error>;

fn main() -> Result<()> {
    let opts = Opt::from_args();

    if env::var_os("RUST_LOG").is_none() {
        env::set_var(
            "RUST_LOG",
            if opts.verbose {
                "{{crate_name}}=debug"
            } else {
                "{{crate_name}}=info"
            },
        );
    }
    tracing_subscriber::fmt::try_init()
        .map_err(|e| anyhow!("Failed to initialize tracing: {}", e))?;

    let config = Config::load(&opts.config_file)?;
    debug!("config loaded: {:?}", config);

    Ok(())
}
