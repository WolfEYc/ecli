extern crate skim;
use crate::cmd::{CommandLookup, LookupVec};
use skim::prelude::*;

pub fn lookup(cmds: LookupVec) -> CommandLookup {
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(true)
        .build()
        .unwrap();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();

    for cmd in cmds.commands {
        tx_item.send(Arc::new(cmd)).unwrap();
    }

    drop(tx_item);

    // `run_with` would read and show items from the stream
    let skim_output = Skim::run_with(&options, Some(rx_item)).unwrap();

    clear_stdout();

    // Welcome to rust!
    skim_output
        .selected_items
        .first()
        .unwrap()
        .as_ref()
        .to_owned()
        .as_any()
        .downcast_ref::<CommandLookup>()
        .unwrap()
        .to_owned()
}

fn clear_stdout() {
    print!("\x1B[2J\x1B[H"); // ANSI escape sequence to clear the screen
}
