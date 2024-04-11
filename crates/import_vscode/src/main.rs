mod convert;

use anyhow::Result;
use clap::Parser;
use serde::Deserialize;
use std::{fs::File, io::Read};
use tracing::{debug, error, info, span, warn, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Arguments {
    /// Input vscode theme file
    input: String,
    /// Output logviewer theme file
    output: String,
}

#[derive(Debug, Deserialize)]
struct VsCodeTheme {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub _type: Option<String>,
    pub colors: vscode_theme::Colors,
}

fn main() -> Result<()> {
    init_log();
    let args = Arguments::parse();
    let input_file = File::open(args.input).or_else(|err| {
        error!("Failed to open input file: {}", err);
        Err(err)
    })?;

    let vscode_theme: VsCodeTheme = serde_json_lenient::from_reader(input_file).or_else(|err| {
        error!("Failed to parse vscode theme: {}", err);
        Err(err)
    })?;

    info!(
        message = "Successfully parsed vscode theme",
        name = vscode_theme.name
    );
    Ok(())
}

fn init_log() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .with_file(true)
        .with_line_number(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
