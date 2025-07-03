mod cli;
mod files;

use clap::Parser;

fn main() {
    let args = cli::Args::parse();

    if !args.gitignore.is_empty() {
        files::generate_gitignore(&args.gitignore);
    }

    if let Some(license) = args.license {
        files::generate_license(&license);
    }

    if args.readme {
        files::generate_readme();
    }

    if args.env_example {
        files::generate_env_example();
    }
}
