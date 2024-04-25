use std::path::PathBuf;

use crate::collection::tag;

pub struct File {
    tags: Vec<tag::Tag>,
    path: PathBuf,
}
