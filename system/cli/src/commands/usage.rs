use {super::Command, crate::UsageError};

const INSTRUCTION: &str = r"AKAIA OS CLI

Usage: akaia [--debug] SUBCOMMAND

Options:
	--debug	Print supplied arguments after execution.

SUBCOMMAND is one of:
	usage
	hello [--name NAME]
	near [...NEAR_CLI_ARGS]
";

pub fn execute<'a>() -> Result<Command<'a>, UsageError<'a>> {
	println!("{}", INSTRUCTION);

	Ok(Command::Usage)
}
