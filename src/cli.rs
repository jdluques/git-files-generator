use clap::{ Parser };

use crate::files::{
    gitignore::GitIgnoreType,
    license::LicenseType,
};

#[derive(Parser)]
pub struct Args {
    #[arg(long, help="Generate env.example. file based on project's .env file")]
    pub env_example: bool,

    #[arg(long, visible_alias="ignore", num_args = 1.., help="Generate .gitignore file")]
    pub gitignore: Vec<GitIgnoreType>,

    #[arg(long, help="Generate LICENSE file")]
    pub license: Option<LicenseType>,

    #[arg(long, help="Generate README.md file with project name")]
    pub readme: bool,
}
