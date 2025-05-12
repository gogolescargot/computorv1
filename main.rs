use std::env;
use regex::Regex;

fn valid_element(element: &str) -> bool
{
    let re = Regex::new(r"^(\+|\-|\*|\/|X\^[0-2])$").unwrap();
	re.is_match(element) || element.parse::<f64>().is_ok() 
}

fn parse(expression: &String)
{
	let elements = expression.split_whitespace();
	for elem in elements
	{
		if !valid_element(elem)
		{
			println!("Error: Parsing '{}'", elem);
			return;
		}
	}
}

fn computor()
{
	let arg: Vec<String> = env::args().collect();
	if arg.len() != 2 {
		println!("Usage: computor <expression>");
		return;
	}
	let expression: &String = &arg[1];
	parse(expression);
}

fn main()
{
	computor();
}