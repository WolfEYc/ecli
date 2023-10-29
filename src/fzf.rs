extern crate skim;
use crate::cmd::CommandLookup;
use skim::prelude::*;

pub fn lookup(cmds: Vec<CommandLookup>) -> CommandLookup {
    let options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .multi(true)
        .build()
        .unwrap();

    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();
    for cmd in cmds {
        tx_item.send(Arc::new(cmd)).unwrap();
    }

    // `run_with` would read and show items from the stream
    let skim_output = Skim::run_with(&options, Some(rx_item)).unwrap();

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
