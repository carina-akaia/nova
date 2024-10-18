use {getargs::{Error, Opt},
     thiserror::Error};

#[derive(Error, Debug)]
pub enum UsageError<'a> {
	#[error("{0}")]
	Getargs(getargs::Error<&'a str>),
	#[error("unknown option {0}")]
	UnknownOption(Opt<&'a str>),
	#[error("unknown command {0:?}")]
	UnknownCommand(&'a str),
	#[error("missing command")]
	MissingCommand,
}

impl<'a> From<getargs::Error<&'a str>> for UsageError<'a> {
	fn from(error: Error<&'a str>) -> Self {
		UsageError::Getargs(error)
	}
}
