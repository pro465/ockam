use crate::{util::exitcode, CommandGlobalOpts};
use clap::Args;

#[derive(Clone, Debug, Args)]
pub struct GetCommand {
    /// Alias name of the node
    pub alias: String,
}

impl GetCommand {
    pub fn run(opts: CommandGlobalOpts, command: GetCommand) {
        let lookup = opts.config.get_lookup();
        match lookup.get_node(&command.alias) {
            Some(addr) => {
                println!("Node: {}\nAddress: {}", command.alias, addr);
            }
            None => {
                eprintln!(
                    "Alias {} not known.  Add it first with `ockam alias set`!",
                    command.alias
                );
                std::process::exit(exitcode::DATAERR);
            }
        }
    }
}
