#![deny(missing_docs)]
//! This crate provides functionality to compare JSON files
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
/// Defines the command line arguments that can be used with this program
pub enum Options {
    #[command(alias = "v")]
    /// Checks whether the keys in the source file/directory align with the base file
    Verify {
        /// This file will be used as base template for every source file
        base: PathBuf,
        /// This can be a file or directory. It will be checked against the base
        source: PathBuf,
        /// Wether to explore the specified source directory recursivly or not
        /// This has no effect when source is a file
        #[arg(long, short, default_value_t = false)]
        recursive: bool,
        /// When this flag is set, the order is also considered while validating
        #[arg(long, short, default_value_t = false)]
        strict: bool,
    },
    /// Checks whether the used templates in the source file/directory align with the base file
    #[command(alias = "vt")]
    VerifyTemplates {
        /// This file will be used as base template for every source file
        base: PathBuf,
        /// This can be a file or directory. It will be checked against the base
        source: PathBuf,
        /// Wether to explore the specified source directory recursivly or not
        /// This has no effect when source is a file
        #[arg(long, short, default_value_t = false)]
        recursive: bool,
    },
    /// Sorts the keys in source based on the order used in base
    #[command(alias = "s")]
    Sort {
        /// This file will be used as base template for every source file
        base: PathBuf,
        /// This can be a file or directory. It will be checked against the base
        source: PathBuf,
        /// Wether to explore the specified source directory recursivly or not
        /// This has no effect when source is a file
        #[arg(long, short, default_value_t = false)]
        recursive: bool,
    },
    /// This will find all differences between base and source
    #[command(alias = "d")]
    Diff {
        /// This file will be used as base template for every source file
        base: PathBuf,
        /// This can be a file or directory. It will be checked against the base
        source: PathBuf,
        /// Wether to explore the specified source directory recursivly or not
        /// This has no effect when source is a file
        #[arg(long, short, default_value_t = false)]
        recursive: bool,
        /// When this flag is active the source will be extended/trimmed based on the base.
        /// If not supplied, a report will be printed, what is more or missing based on the base.
        #[arg(long, short, default_value_t = false)]
        fix: bool,
        /// When supplied, the output will be written to this file instead of stdout.
        #[arg(long, short)]
        output: Option<PathBuf>,
    },
}
fn main() {
    let options = Options::parse();
    match options {
        Options::Diff {
            base,
            source,
            fix,
            output,
            recursive,
        } => diff::diff(base, source, recursive, fix, output),
        Options::Verify {
            base,
            source,
            recursive,
            strict,
        } => verify::verify(base, source, recursive, strict),
        Options::VerifyTemplates {
            base,
            source,
            recursive,
        } => verify_templates::verify_templates(base, source, recursive),
        Options::Sort {
            base,
            source,
            recursive,
        } => sort::sort(base, source, recursive),
    }
}
mod diff;
mod sort;
mod utility;
mod verify;
mod verify_templates;
