// SPDX-License-Identifier: BSD-3-Clause
// Copyright (C) 2025 Affan Ahmad <st_iaffan@Outlook.com>

///
/// Flerry Compiler 0.0.1dev1
/// -
///
/// ### What is Flerry?
/// Flerry is a simple language for a simple playground.
///
/// To build it, all you have to do is install git, cargo and rustc according to your
/// Operating System's package management system. Then you have to clone this repository
/// to your home directory. In Windows, this is `C:\Users\<your_user_name>` while in Unix
/// like systems it is normally `/home/<your_user_name>`. Look into the manual of your
/// Operating System and search where the `user` directory normally resides.
///
/// Run the following command when you have decided where you want to clone the repository:
/// ```bash
/// git clone https://github.com/rosewud-lang/rosewud.git <insert_path>
/// ```
///
/// Then move into the directory and run:
/// ```bash
/// cargo build
/// ```
///
/// The resulting binary will be placed in ./target/
/// Happy hacking!
///
mod cli;
mod utils;

fn main() {
    cli::cli();
}
