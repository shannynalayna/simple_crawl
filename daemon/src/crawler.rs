// Implementation of a simple link crawler within a single domain

use std::collections::VecDeque;
use std::time::Duration;

use anyhow::{Error, Result};
use reqwest::Client;
use select::{document::Document, predicate::Name};
use tokio::{sync::watch, task, time};

use crawl_daemon::tree::{Branch, Tree};

pub async fn crawl(start: String, shutdown_rx: watch::Receiver<()>) -> Result<Tree> {
    // Instantiate a tree to contain spider results
    let mut tree = Tree::default();
    let root_domain = start;

    // Create a shareable client for spiders
    let spider_client = Client::new();

    // Data stored to help the crawler proceed
    let mut visited_links = vec![];
    let mut links_to_visit = VecDeque::new();
    links_to_visit.push_back(root_domain);
    let mut spiders = VecDeque::new();

    loop {
        // Spawn new spiders, based on the links we need to visit
        while let Some(link) = links_to_visit.pop_front() {
            spiders.push_back(task::spawn(branch_search(link, spider_client.clone())));
        }

        while let Some(spider) = spiders.pop_front() {
            if let Ok(br) = spider.await? {
                // Ensure other spiders do not repeat this one's work
                visited_links.push(br.root().to_owned());
                for leaf in br.leaves() {
                    if !visited_links.contains(leaf) {
                        links_to_visit.push_back(leaf.clone());
                    }
                }
                // Store this spider's results
                tree = tree.insert_branch(br);
                // Remove spider from our list of spawned tasks
            }
        }

        // Check if we have received a shutdown message, in which case we will cease spawning spiders
        if shutdown_rx.has_changed()? {
            break;
        }

        time::sleep(Duration::from_secs(1)).await;
    }

    drop(shutdown_rx);

    Ok(tree)
}

// Retrieve self-referencing links from a starting domain
pub async fn branch_search(link: String, client: Client) -> Result<Branch> {
    // Create a searchable link from input
    let searchable_link = inspect_root(link);
    println!("Branching search at {searchable_link}");
    // Return HTML text from link
    let res = client.get(&searchable_link).send().await?.text().await?;
    if res.is_empty() {
        return Err(Error::msg("Crawler could not find remote data"));
    }

    // Retrieve self-referencing links from HTML & validate
    //  the format of the retrieved links
    let next_urls: Vec<_> = Document::from(res.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .map(|val| inspect_leaf(&searchable_link, val))
        .collect();

    Ok(Branch::new(searchable_link, next_urls))
}

// Ensure the root link is a correctly formatted URL
fn inspect_root(root: String) -> String {
    if !root.starts_with("https://www.") {
        let prefix = "https://www.".to_owned();
        return prefix + &root;
    }
    root
}

// Ensure leaf links include the root domain
fn inspect_leaf(root: &str, link: &str) -> String {
    if link.starts_with('/') {
        let root_domain = root.trim_end_matches('/').to_owned();
        return root_domain + link;
    }
    link.to_owned()
}
