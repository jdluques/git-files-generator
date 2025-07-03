mod cli;
mod files;
mod http_client;
mod utils;

use clap::Parser;

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Args::parse();

    if !args.gitignore.is_empty() {
        files::generate_gitignore(&args.gitignore).await?;
    }

    if let Some(license_type) = &args.license {
        files::generate_license(&license_type).await?;
    }

    if args.readme {
        files::generate_readme()?;
    }

    if args.env_example {
        files::generate_env_example();
    }

    Ok(())
}
