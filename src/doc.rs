
use std::path::{PathBuf};

pub struct Entry {
   title: String,
   url: String,
}

pub struct Document {
   filename: PathBuf,
   entries: Vec<Entry>,
}