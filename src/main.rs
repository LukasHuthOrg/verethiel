use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
enum Options {
    #[command(alias = "v")]
    Verify {
        base: PathBuf,
        source: PathBuf,
        #[arg(long, short, default_value_t = false)]
        recursive: bool,
    },
    #[command(alias = "vt")]
    VerifyTemplates {
        base: PathBuf,
        source: PathBuf,
        #[arg(long, short, default_value_t = false)]
        recursive: bool,
    },
    #[command(alias = "d")]
    Diff {
        base: PathBuf,
        source: PathBuf,
        #[arg(long, short, default_value_t = false)]
        fix: bool,
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
        } => diff::diff(base, source, fix, output),
        Options::Verify {
            base,
            source,
            recursive,
        } => verify::verify(base, source, recursive),
        Options::VerifyTemplates {
            base,
            source,
            recursive,
        } => verify_templates::verify_templates(base, source, recursive),
    }
}
mod diff;
mod utility;
mod verify;
mod verify_templates;
