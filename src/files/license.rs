use clap::ValueEnum;

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
    #[value(name = "lgpl-3.0", aliases = ["LGPL", "lgpl"])]
    LGPL,
    #[value(name = "agpl-3.0", aliases = ["AGPL", "agpl"])]
    AGPL,
    #[value(name = "bsd-3-clause", aliases = ["BSD", "bsd", "BSD3", "bsd3"])]
    BSD,
}

pub fn generate(license_type: &LicenseType) {
    println!("Generate LICENSE file");
    dbg!(license_type);
}
