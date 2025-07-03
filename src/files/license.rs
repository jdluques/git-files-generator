use clap::ValueEnum;
use std::{
    fmt::{ self, Display, Formatter },
    error::Error,
};

use crate::{
    files,
    http_client,
    utils,
};

#[derive(Debug, Clone, ValueEnum)]
pub enum LicenseType {
    #[value(name = "mit", alias = "MIT")]
    MIT,
    #[value(name = "apache-2.0", aliases = ["Apache2", "APACHE2", "Apache", "APACHE"])]
    Apache2,
    #[value(name = "gpl-2.0", aliases = ["GPLv2", "GPL2", "GNUv2", "GNUV2", "gnu-v2", "GNU-V2"])]
    GNUv2,
    #[value(name = "gpl-3.0", aliases = ["GPLv3", "GPL3", "GNUv3", "GNUV3", "gnu-v3", "GNU-V3"])]
    GNUv3,
    #[value(name = "lgpl-2.1", aliases = ["LGPL", "lgpl"])]
    LGPL,
    #[value(name = "agpl-3.0", aliases = ["AGPL", "agpl"])]
    AGPL,
    #[value(name = "bsd-2-clause", aliases = ["BSD2", "bsd2"])]
    BSD2,
    #[value(name = "bsd-3-clause", aliases = ["BSD3", "bsd3"])]
    BSD3,
}

impl Display for LicenseType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            LicenseType::MIT => "mit",
            LicenseType::Apache2 => "apache-2.0",
            LicenseType::GNUv2 => "gpl-2.0",
            LicenseType::GNUv3 => "gpl-3.0",
            LicenseType::LGPL => "lgpl-2.1",
            LicenseType::AGPL => "agpl-3.0",
            LicenseType::BSD2 => "bsd-2-clause",
            LicenseType::BSD3 => "bsd-3-clause",
        };
        write!(f, "{}", s)
    }
}

pub async fn generate(license_type: &LicenseType) -> Result<(), Box<dyn Error>> {
    let license_type_str = license_type.to_string();
    let file_content = http_client::fetch_template(
        &crate::files::FileType::License,
        license_type_str.as_str()
    ).await.unwrap();

    utils::create_file(files::FileType::License.to_string().as_str(), &file_content)?;

    Ok(())
}
