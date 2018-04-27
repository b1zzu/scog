use core::context::Context;
use utils::error::Error;

pub fn exec(context: &mut Context, args: &mut Vec<String>) -> Result<(), Error> {
    let branch_name = match args.get(0) {
        Some(b) => b,
        None => {
            let error = format!(
                "\
'checkout' requires BRANCH argument.
Usage: bog checkout BRANCH.
                "
            );
            return Err(error)?;
        }
    };

    context.repository().checkout(branch_name)
}