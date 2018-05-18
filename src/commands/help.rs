use core::context::Context;
use utils::error::Error;

pub fn exec(_: &mut Context, _: &mut Vec<String>) -> Result<(), Error> {
    println!(
        "\
Usage: scog COMMAND [ARGS]

Command:
    clone           ...
    checkout        ...
    pull            ...
    push            ...
        "
    );
    Ok(())
}