use std::path::Path;

use anyhow::Result;
use clap::Parser;
use tokio::fs;

use json_files_combiner::cli::Args;
use json_files_combiner::combiner::combiner;

#[tokio::main]
async fn main() -> Result<()> {
    let commands = Args::parse();

    if let Some(paths) = commands.paths {
        let paths: Vec<&Path> = paths.iter().map(|path| Path::new(path)).collect();

        let result = combiner(&paths).await?;

        match commands.destination {
            Some(destination) => {
                fs::write(&destination, serde_json::to_string_pretty(&result)?).await?
            }
            None => fs::write(".", serde_json::to_string_pretty(&result)?).await?,
        }
    }

    Ok(())
}
