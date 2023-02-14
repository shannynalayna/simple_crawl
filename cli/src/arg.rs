// Data structures related to CLI arguments

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct CrawlerCli {
    #[command(subcommand)]
    pub cmd: CrawlerCommand,
}

#[derive(Subcommand)]
pub enum CrawlerCommand {
    Start { root: String },
    Stop,
    List,
}
