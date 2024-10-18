use {super::Command,
     crate::UsageError,
     getargs::{Opt, Options}};

pub fn execute<'a, I: Iterator<Item = &'a str>>(options: &mut Options<&'a str, I>)
                                                -> Result<Command<'a>, UsageError<'a>> {
	let mut name = None;

	while let Some(option) = options.next_opt()? {
		match option {
			| Opt::Long("name") => name = Some(options.value()?),
			| _ => return Err(UsageError::UnknownOption(option)),
		}
	}

	println!("Hello, {}!", name.unwrap_or("stranger"));

	Ok(Command::Hello { name })
}
