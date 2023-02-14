// Implements a simple CLI for an async web crawler in Rust

mod arg;
mod message;

use std::fs;
use std::io::Write;
use std::net::TcpStream;

use clap::Parser;

use arg::{CrawlerCli, CrawlerCommand};
use crawl_daemon::tree::Tree;
use message::CrawlerMsg;

const CRAWLER_DAEMON: &str = "127.0.0.1:7878";
const CRAWLER_RESULTS_FILE: &str = "crawl_results.txt";

fn main() {
    let cli = CrawlerCli::parse();
    match cli.cmd {
        CrawlerCommand::Start { root } => {
            let mut stream =
                TcpStream::connect(CRAWLER_DAEMON).expect("Unable to connect to crawler_daemon");

            println!("Going to start crawler at {root}");
            // Send message to crawler to start looking at root URL
            let message = CrawlerMsg::Start { root };
            let msg_bytes =
                serde_json::to_vec(&message).expect("Unable to create start message bytes");
            if let Ok(()) = stream.write_all(&msg_bytes) {
                println!("Sent message to crawler");
            } else {
                println!("Failed to send message to crawler");
            }
        }
        CrawlerCommand::Stop => {
            let mut stream =
                TcpStream::connect(CRAWLER_DAEMON).expect("Unable to connect to crawler_daemon");

            println!("Going to stop crawling...");
            let msg_bytes =
                serde_json::to_vec(&CrawlerMsg::Stop).expect("Unable to create stop message bytes");
            if let Ok(()) = stream.write_all(&msg_bytes) {
                println!("Sent shutdown message to crawler");
            } else {
                println!("Failed to send message to crawler");
            }
        }
        CrawlerCommand::List => {
            println!("Going to print tree results");
            let bytes: Vec<u8> =
                fs::read(CRAWLER_RESULTS_FILE).expect("Unable to read crawl results file");
            let tree: Tree =
                serde_json::from_slice(&bytes).expect("Unable to convert bytes to tree");
            println!("{tree:?}");
        }
    }
}
