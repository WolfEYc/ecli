use std::path::PathBuf;
extern crate skim;
use skim::prelude::*;
use std::{fs, io::Cursor};

fn lookup() {
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(true)
        .build()
        .unwrap();

    // `SkimItemReader` is a helper to turn any `BufRead` into a stream of `SkimItem`
    // `SkimItem` was implemented for `AsRef<str>` by default
    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(file_path));

    // `run_with` would read and show items from the stream
    let skim_output = Skim::run_with(&options, Some(items)).unwrap();
    let selected_item = skim_output.selected_items.first().unwrap();

    selected_item.output();

    for item in selected_items.iter() {
        print!("{}{}", item.output(), "\n");
    }
}
