use core::context::Context;
use utils::error::Error;

pub fn exec(context: &mut Context, args: &mut Vec<String>) -> Result<(), Error> {
    let repo = match args.first() {
        None => {
            let error = format!(
                "\
'clone' requires REPO argument.
Usage: scog clone REPO.
                "
            );
            return Err(error)?;
        }
        Some(repo) => repo,
    };

    context.repository().clone(repo.as_str())
}