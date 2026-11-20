//! Ragers: encrypt files in a repository using the `age` format.
//!
//! This library exposes the configuration model, recipient resolution, and the
//! [`run`] entrypoint that the `ragers` binary delegates to. Integration tests in
//! `tests/` exercise these seams directly.

pub mod cli;
pub mod config;
pub mod context;
pub mod error;

use clap::CommandFactory;

use cli::{Cli, cmd};
use config::RawConfig;
use context::Context;
use error::{CmdError, WithPath};

/// Run the CLI given an already-parsed [`Cli`].
///
/// The binary's `main` parses arguments with [`clap`] and delegates here. Kept
/// as a library function so the full pipeline can be driven from tests.
pub fn run(cli: Cli) -> Result<(), CmdError> {
    if cli.command.is_none() {
        Cli::command().print_help().result_with_path(None)?;
        return Ok(());
    }

    let config_path = match &cli.config_path {
        Some(custom) => custom.clone(),
        None => {
            let working_dir = std::env::current_dir().result_with_path(None)?;
            let yaml_ver = working_dir.join(".ragers.yaml");
            let yml_ver = working_dir.join(".ragers.yml");
            if std::fs::exists(&yaml_ver).result_with_path(Some(&yaml_ver))? {
                yaml_ver
            } else if std::fs::exists(&yml_ver).result_with_path(Some(&yml_ver))? {
                yml_ver
            } else {
                return Err(CmdError::Io {
                    source: std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "no config file found",
                    ),
                    path: None,
                });
            }
        }
    };

    let config = RawConfig::new(&config_path)?;

    let ctx = Context::new(cli, &config)?;

    match &ctx.cli.command {
        Some(cli::Commands::Encrypt { files, no_delete }) => cmd::encrypt(&ctx, files, no_delete)?,
        Some(cli::Commands::Decrypt {
            files,
            identity,
            no_delete,
        }) => cmd::decrypt(&ctx, files, identity, no_delete)?,
        Some(cli::Commands::Edit { file, identity }) => cmd::edit(&ctx, file, identity)?,
        None => {
            Cli::command().print_help().result_with_path(None)?;
        }
    }

    Ok(())
}
