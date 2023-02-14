// Implementation of tasks spawned by the crawler

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Tree {
    branches: Vec<Branch>,
}

impl Tree {
    pub fn branches(&self) -> &[Branch] {
        &self.branches
    }

    pub fn insert_branch(mut self, branch: Branch) -> Self {
        self.branches.push(branch);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Branch {
    pub root: String,
    pub leaves: Vec<String>,
}

impl Branch {
    pub fn new(root: String, leaves: Vec<String>) -> Self {
        Self { root, leaves }
    }

    pub fn root(&self) -> &str {
        &self.root
    }

    pub fn leaves(&self) -> &[String] {
        &self.leaves
    }
}
