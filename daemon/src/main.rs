// Implements a web crawler that will create a tree from a single domain root.

pub mod crawler;
mod message;

use std::fs;
use std::io::Read;
use std::net::TcpListener;

use anyhow::Result;
use tokio::sync::watch;

use crawl_daemon::tree::Tree;

use crawler::crawl;
use message::CrawlerMsg;

pub const CRAWLER_SOCKET: &str = "127.0.0.1:7878";
pub const CRAWLER_RESULT_FILE: &str = "crawl_results.txt";

fn main() {
    if let Err(e) = run_crawler() {
        eprintln!("Failed to run crawler: {e}");
        std::process::exit(1);
    }
}

#[tokio::main]
async fn run_crawler() -> Result<()> {
    // Connect to socket to listen for incoming instructions
    let command_listener = TcpListener::bind(CRAWLER_SOCKET)?;

    let mut crawler_handle = None;
    let (shutdown_tx, shutdown_rx) = watch::channel(());

    for stream in command_listener.incoming() {
        let mut command_buf = vec![];
        let mut stream = stream?;
        stream.read_to_end(&mut command_buf)?;
        let cmd: CrawlerMsg = serde_json::from_slice(&command_buf)?;
        match cmd {
            CrawlerMsg::Start { root } => {
                // Start the crawler, making sure we don't already have one running
                if crawler_handle.is_some() {
                    eprintln!("There is already a crawler running...");
                } else {
                    println!("Starting to crawl {root}");
                    crawler_handle = Some(tokio::spawn(crawl(root, shutdown_rx.clone())));
                }
            }
            CrawlerMsg::Stop => {
                println!("Going to stop the crawler...");
                if crawler_handle.is_some() {
                    // Send shutdown message to in-flight crawler
                    shutdown_tx.send(())?;
                    // Take the in-flight handle to retrieve results
                    let handle = crawler_handle.take().unwrap();
                    // Write results to a file
                    let resulting_tree: Tree = handle.await??;
                    let tree_json_bytes = serde_json::to_vec(&resulting_tree)?;
                    fs::write(CRAWLER_RESULT_FILE, &tree_json_bytes)?;
                    println!("Sent results to {CRAWLER_RESULT_FILE}");
                } else {
                    eprintln!("No crawler to stop");
                }
            }
        }
    }

    Ok(())
}
