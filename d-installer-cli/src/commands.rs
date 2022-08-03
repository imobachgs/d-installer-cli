use clap::{Args, Subcommand};

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Manage storage options
    Storage(StorageArgs),
    /// Manage language options
    Language(LanguageArgs),
    /// Manage software options
    Software(SoftwareArgs),
}

#[derive(Args, Debug)]
pub struct StorageArgs {
    #[clap(subcommand)]
    pub command: StorageCommands,
}

#[derive(Subcommand, Debug)]
pub enum StorageCommands {
    /// List the storage actions to perform
    Actions,
    /// List available devices for installation
    AvailableDevices,
    /// Select devices for installation
    SelectDevices { devices: Vec<String> },
}

#[derive(Args, Debug)]
pub struct LanguageArgs {
    #[clap(subcommand)]
    pub command: LanguageCommands,
}

#[derive(Subcommand, Debug)]
pub enum LanguageCommands {
    /// List available languages for installation
    Available,
    /// Select the language to install in the target system
    Select { langs: Vec<String> },
}

#[derive(Args, Debug)]
pub struct SoftwareArgs {
    #[clap(subcommand)]
    pub command: SoftwareCommands,
}

#[derive(Subcommand, Debug)]
pub enum SoftwareCommands {
    /// List available base products
    Products,
    /// Select the product to install in the target system
    SelectProduct { id: String },
}
