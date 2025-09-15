// SPDX-License-Identifier: MIT
// Copyright (C) 2025 Affan Ahmad <st_iaffan@Outlook.com>

use clap::builder::styling::{AnsiColor, Style};
use clap::Parser;
use std::path::Path;

// Configures the style for the CLI.
const STYLES: clap::builder::styling::Styles = clap::builder::styling::Styles::styled()
    .header(Style::new().fg_color(Some(clap::builder::styling::Color::Ansi(AnsiColor::Yellow))))
    .usage(Style::new().fg_color(Some(clap::builder::styling::Color::Ansi(AnsiColor::Yellow))))
    .literal(Style::new().fg_color(Some(clap::builder::styling::Color::Ansi(AnsiColor::Green))))
    .error(Style::new().fg_color(Some(clap::builder::styling::Color::Ansi(AnsiColor::Red))));

#[derive(Parser)]
#[command(author, version, about, long_about = None, styles = STYLES)]
pub struct Cli {
    /// The path to the file to run
    #[arg(value_name = "path")]
    pub path: String,
}

pub fn read_file(path: &Path) -> std::io::Result<String> {
    let content = std::fs::read_to_string(path)?;
    Ok(content.trim_end().to_string())
}

pub fn run() {
    flerry_compiler::compiler::compile();
}
