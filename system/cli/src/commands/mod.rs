pub mod hello;
pub mod near;
pub mod usage;

#[derive(Debug)]
pub enum Command<'a> {
	Usage,
	Hello { name: Option<&'a str> },
	Near { args: Vec<&'a str> },
}
