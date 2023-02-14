// Provides serializable commands for the crawler

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum CrawlerMsg {
    Start { root: String },
    Stop,
}
