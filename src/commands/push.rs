use core::context::Context;
use utils::error::Error;

pub fn exec(context: &mut Context, _: &mut Vec<String>) -> Result<(), Error> {
    context.repository().push()
}
