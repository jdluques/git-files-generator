mod cli;
mod files;
mod http_client;
mod utils;

use clap::Parser;
use tokio::join;

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Args::parse();

    let gitignore_fut = async {
        if let Some(git_ignore_types) = &args.gitignore {
            files::generate_gitignore(&git_ignore_types, args.append).await
        } else {
            Ok(())
        }
    };

    let license_fut = async {
        if let Some(license_type) = &args.license {
            files::generate_license(&license_type).await
        } else {
            Ok(())
        }
    };
    
    let env_example_fut = async {
        if args.env_example {
            files::generate_env_example()
        } else {
            Ok(())
        }
    };
    
    let (gitignore_res, license_res, env_example_res) = join!(gitignore_fut, license_fut, env_example_fut);

    gitignore_res?;
    license_res?;
    env_example_res?;

    if args.readme {
        files::generate_readme()?;
    }

    Ok(())
}
