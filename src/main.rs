use std::{fs::read_to_string, path::PathBuf};

use glob::glob;

use crate::model::Events;

mod model;

fn main() -> anyhow::Result<()> {
    for path in glob("data/**/*.hcl")?.flatten().collect::<Vec<PathBuf>>() {
        let content = read_to_string(path)?;

        let events = hcl::from_str::<Events>(&content)?;

        println!("{events:?}");
    }

    Ok(())
}
