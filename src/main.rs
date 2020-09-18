#![deny(warnings, missing_debug_implementations, rust_2018_idioms)]

use std::{env, fs};

use clap::Clap;
use eyre::{eyre, WrapErr};
use rson_rs as rson;
use serde_derive::Deserialize;
use tracing::debug;
use tracing_subscriber;

#[derive(Debug, Clap)]
struct Args {
    #[clap(
        short = "c",
        long = "config-file",
        default_value = "{{project-name}}.rson"
    )]
    /// Configuration file to use.
    config_file: String,
    #[clap(short = "v")]
    /// If specified, debug output will be enabled.
    verbose: bool,
}

#[derive(Debug, Deserialize)]
struct Config {}

impl Config {
    pub fn load(path: &str) -> Result<Config> {
        let config_file = fs::read_to_string(path)
            .map_err(|e| eyre!("unable to read configuration file: {}", e))?;
        rson::de::from_str(&config_file)
            .map_err(|e| eyre!("invalid config file {}: {}", path, e))
            .and_then(Config::validate)
    }

    fn validate(config: Config) -> Result<Config> {
        Ok(config)
    }
}

type Error = eyre::Error;
type Result<T, E = Error> = std::result::Result<T, E>;

fn init_tracing(args: &Args) -> Result<()> {
    use tracing_subscriber::{filter::EnvFilter, fmt};

    let filter = if env::var_os("RUST_LOG").is_some() {
        EnvFilter::try_from_default_env().map_err(Into::into)
    } else {
        EnvFilter::try_new(&format!(
            "{{crate_name}}={}",
            if args.verbose { "debug" } else { "info" }
        ))
        .map_err::<Error, _>(Into::into)
    }
    .wrap_err_with(|| eyre!("failed to initialize tracing EnvFilter"))?;

    fmt::fmt()
        .with_env_filter(filter)
        .try_init()
        .map_err(|e| eyre!("Failed to initialize tracing: {}", e))
}

fn main() -> Result<()> {
    let args = Opt::parse();

    init_tracing(&args)?;

    let config = Config::load(&args.config_file)?;
    debug!("config loaded: {:?}", config);

    Ok(())
}
