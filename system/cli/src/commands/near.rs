use {super::Command, crate::UsageError, getargs::Options};

pub fn execute<'a, I: Iterator<Item = &'a str>>(options: &mut Options<&'a str, I>)
                                                -> Result<Command<'a>, UsageError<'a>> {
	let args = options.positionals().collect();

	// Then we call near-cli-rs here... I guess?
	println!("{}", near_cli_rs::common::get_near_exec_path());

	Ok(Command::Near { args })
}
