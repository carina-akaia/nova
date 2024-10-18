mod commands;
mod errors;

use {commands::{hello, near, usage, Command},
     errors::UsageError,
     getargs::{Opt, Options},
     std::process::exit};

#[allow(unused)] // `command` is marked as unused for some reason.
#[derive(Debug)]
struct Arguments<'a> {
	debug:   bool,
	command: Command<'a>,
}

fn execute_command<'a, I: Iterator<Item = &'a str>>(options: &mut Options<&'a str, I>)
                                                    -> Result<Arguments<'a>, UsageError<'a>> {
	let mut debug = false;

	while let Some(opt) = options.next_opt()? {
		match opt {
			| Opt::Long("debug") => debug = true,
			| _ => return Err(UsageError::UnknownOption(opt)),
		}
	}

	let command_name = options.next_positional()
	                          .ok_or(UsageError::MissingCommand)?;

	let command = match command_name {
		| "usage" => usage::execute()?,
		| "hello" => hello::execute(options)?,
		| "near" => near::execute(options)?,
		| _ => return Err(UsageError::UnknownCommand(command_name)),
	};

	Ok(Arguments { debug, command })
}

fn main() {
	let args: Vec<_> = std::env::args().skip(1).collect();

	let mut options = Options::new(args.iter().map(String::as_str));

	match execute_command(&mut options) {
		| Ok(arguments) => {
			if arguments.debug {
				println!("\n{arguments:#?}")
			}
		},

		| Err(error) => {
			println!("Usage error: {error}\n");
			usage::execute().ok();
			exit(1);
		},
	}
}
