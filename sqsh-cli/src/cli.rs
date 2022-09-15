use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// Command-line Interface (CLI) for the sqsh library
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Subcommand to be executed
    #[clap(subcommand)]
    pub command: Commands,

    #[clap(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,
}

/// Commands to be executed by the CLI
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Duplicate the input to the output
    Duplicate {
        /// Input file
        #[clap(value_parser)]
        input: PathBuf,

        /// Output file
        #[clap(value_parser)]
        output: Option<PathBuf>,
    },
    /// Calculate Adler32 checksum
    Adler32 {
        /// Input file
        #[clap(value_parser)]
        input: PathBuf,
    },
    /// Calculate CRC32 checksum
    CRC32 {
        /// Input file
        #[clap(value_parser)]
        input: PathBuf,
    },
}
