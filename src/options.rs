#[derive(Clone)]
pub enum Command {
    None,
    Clone,
    Checkout,
    Pull,
    Push,
}

pub struct Options {
    command: Command,
    repo: String,
    branch: String,
    help: bool,
}

impl Options {
    fn new() -> Options {
        Options {
            command: Command::None,
            repo: String::new(),
            branch: String::new(),
            help: false,
        }
    }

    pub fn get_command(&self) -> &Command {
        &self.command
    }

    pub fn get_help(&self) -> &bool {
        &self.help
    }

    pub fn get_repo(&self) -> &String {
        &self.repo
    }

    pub fn get_branch(&self) -> &String {
        &self.branch
    }

    pub fn parse(args: Vec<String>) -> Result<Options, String> {
        let mut options = Options::new();

        // first ( 0 ) arguments is the name of the program
        let mut i: usize = 1;
        while i < args.len() {
            match args[i].as_str() {
                "--help" => options.help = true,
                "clone" => {
                    options.command = Command::Clone;

                    i = i + 1;
                    if i >= args.len() {
                        let err = format!("'bog clone' requires REPO argument.");
                        let err = format!("{}\n Usage: bog clone REPO.", err);
                        return Err(err);
                    }

                    options.repo = args[i].clone();
                },
                "checkout" => {
                    options.command = Command::Checkout;

                    i = i + 1;
                    if i >= args.len() {
                        let err = format!("'bog checkout' requires BRANCH argument.");
                        let err = format!("{}\n Usage: bog checkout BRANCH.", err);
                        return Err(err);
                    }

                    options.branch = args[i].clone();
                },
                "pull" => {
                    options.command = Command::Pull;
                },
                "push" => {
                    options.command = Command::Push;
                },

                &_ => {
                    let err = format!("'{}' is not a valid command", args[i]);
                    return Err(err);
                }
            }
            i = i + 1;
        }

        Ok(options)
    }
}
