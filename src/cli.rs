use clap::{ Parser, ArgAction };

use crate::files::{
    gitignore::GitIgnoreType,
    license::LicenseType,
};

#[derive(Parser)]
pub struct Args {
    #[arg(long="env-example", short='e', help="Generate env.example. file based on project's .env file")]
    pub env_example: bool,

    #[arg(long="append", short='a', help="Append to the .gitignore file rather than write/overwrite it")]
    pub append: bool,

    #[arg(long="gitignore", short='g', visible_alias="ignore", num_args = 0.., action = ArgAction::Set, help="Generate .gitignore file")]
    pub gitignore: Option<Vec<GitIgnoreType>>,

    #[arg(long="license", short='l', help="Generate LICENSE file")]
    pub license: Option<LicenseType>,

    #[arg(long="readme", short='r', help="Generate README.md file with project name")]
    pub readme: bool,
}
