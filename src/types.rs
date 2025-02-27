use indexmap::IndexMap;
use std::collections::HashMap;

pub type VersionMap = IndexMap<String, HashMap<String, String>>;

#[derive(Debug)]
pub struct ExtVersion {
    pub version: String,
    pub target_platform: Option<String>,
    pub file_source: String,
}

pub enum SelectMode {
    SpecVersion(String),
    NLatest(usize),
}
