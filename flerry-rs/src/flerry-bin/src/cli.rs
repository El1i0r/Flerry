// SPDX-License-Identifier: BSD-3-Clause
// Copyright (C) 2025 Affan Ahmad <st_iaffan@Outlook.com>

use crate::utils::cli_utils::{Cli, read_file, run};
use clap::Parser;
use colored::*;
use std::path::Path;

pub fn cli() {
    match Cli::try_parse() {
        Ok(cli) => {
            let path = Path::new(&cli.path);

            match read_file(path) {
                Ok(content) => {
                    println!(
                        "{}{}{}{}",
                        "[SUCCESS]".green(),
                        " Debug:".magenta(),
                        " File read correctly, will attempt to compile now\n",
                        content
                    );

                    // Compile
                    run();
                }
                Err(e) => {
                    eprintln!(
                        "{}{}{}{}{}",
                        "[FAILURE]".red(),
                        " Debug:".magenta(),
                        " File was not processed correctly or does not exist.\n",
                        "\nError: ".red(),
                        e,
                    );
                    std::process::exit(64);
                }
            }
        }

        Err(e) => {
            if e.kind() == clap::error::ErrorKind::MissingRequiredArgument {
                eprintln!(
                    "{}{}",
                    "Error: ".red(),
                    "No input files.\n\nPlease specify the path to the file you want to compile."
                );
                std::process::exit(1);
            } else {
                e.exit();
            }
        }
    }
}
