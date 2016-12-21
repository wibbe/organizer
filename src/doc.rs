
use std::path::{PathBuf};

pub struct Entry {
   title: String,
   url: String,
}

pub struct Document {
   filename: Option<PathBuf>,
   entries: Vec<Entry>,
}

impl Default for Document {
   fn default() -> Document {
      Document {
         filename: None,
         entries: Vec::new(),
      }
   }
}