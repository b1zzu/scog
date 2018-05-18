use core::context::Context;
use utils::error::Error;

pub mod clone;
pub mod checkout;
pub mod help;
pub mod pull;
pub mod push;

pub fn exec(cmd: &str) -> Option<fn(&mut Context, &mut Vec<String>) -> Result<(), Error>> {
    let f = match cmd {
        "clone" => clone::exec,
        "checkout" => checkout::exec,
        "help" => help::exec,
        "pull" => pull::exec,
        "push" => push::exec,
        _ => return None,
    };
    Some(f)
}