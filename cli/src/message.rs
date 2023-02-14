// Serializable data structure used to send Crawler commands

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum CrawlerMsg {
    Start { root: String },
    Stop,
}
