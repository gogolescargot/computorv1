use std::env;
use std::cmp;
use regex::Regex;

#[derive(Debug)]
pub enum EXPR {
	Variable(f64),
	Sign(char),
	Exponent(i8),
}

fn parse(expression: &String, parsed_expressions: &mut Vec<EXPR>) -> Result<(), String> {
	let elements = expression.split_whitespace();
	let re_sign = Regex::new(r"^(\+|\-|\*|\/|\=)$").unwrap();
	let re_exponent = Regex::new(r"^X\^[0-2]$").unwrap();

	for elem in elements {
		if re_exponent.is_match(elem) {
			let power_char = elem.chars().nth(2).unwrap();
			let power: i8 = (power_char as i8) - ('0' as i8);
			parsed_expressions.push(EXPR::Exponent(power));
		} else if re_sign.is_match(elem) {
			let sign_char = elem.chars().next().unwrap();
			parsed_expressions.push(EXPR::Sign(sign_char));
		} else if let Ok(val) = elem.parse::<f64>() {
			parsed_expressions.push(EXPR::Variable(val));
		} else {
			return Err(format!("Error: Parsing element '{}' is not supported or invalid.", elem));
		}
	}
	Ok(())
}

fn validate(parsed_expressions: &Vec<EXPR>) -> Result<(), String> {
	if parsed_expressions.is_empty() {
		return Err("Error: Expression is empty.".to_string());
	}
	if let EXPR::Sign(_) = parsed_expressions.first().unwrap() {
		return Err("Error: Expression starts with a sign.".to_string());
	}
	if let EXPR::Sign(_) = parsed_expressions.last().unwrap() {
		return Err("Error: Expression ends with a sign.".to_string());
	}

	let mut equal_count = 0;
	let mut x2_count = 0;
	let mut x1_count = 0;
	let mut x0_count = 0;

	for i in 0..parsed_expressions.len() {
		match parsed_expressions[i] {
			EXPR::Sign('=') => {
				equal_count += 1;
				if equal_count > 1 {
					return Err("Error: Multiple '=' signs found.".to_string());
				}
				if x2_count > 1 || x1_count > 1 || x0_count > 1 {
					return Err("Error: Multiple 'X^N' terms found.".to_string());
				}
				x2_count = 0;
				x1_count = 0;
				x0_count = 0;
			}
			EXPR::Sign(_) => {
				if i > 0 {
					if let EXPR::Sign(_) = parsed_expressions[i - 1] {
						return Err("Error: Two consecutive signs found.".to_string());
					}
				}
			}
			EXPR::Variable(_) => {
				if i > 0 {
					match parsed_expressions[i - 1] {
						EXPR::Variable(_) | EXPR::Exponent(_) => {
							return Err("Error: Two consecutive numbers found.".to_string());
						}
						_ => {}
					}
				}
			}
			EXPR::Exponent(exp) => {
				if i > 0 {
					match parsed_expressions[i - 1] {
						EXPR::Variable(_) | EXPR::Exponent(_) => {
							return Err("Error: Two consecutive numbers found.".to_string());
						}
						_ => {}
					}
				}
				match exp {
					2 => x2_count += 1,
					1 => x1_count += 1,
					0 => x0_count += 1,
					_ => {}
				}
			}
		}
	}

	if x2_count > 1 || x1_count > 1 || x0_count > 1 {
		return Err("Error: Multiple 'X^N' terms found.".to_string());
	}

	Ok(())
}

fn degree(parsed_expressions: &Vec<EXPR>) -> i8
{
	let mut degree = 0;

	for elem in parsed_expressions {
		match elem {
			EXPR::Exponent(exp) => {
				degree = cmp::max(degree, *exp);
			}
			_ => {}
		}
	}

	return degree
}

fn computor()
{
	let arg: Vec<String> = env::args().collect();
	if arg.len() != 2 {
		println!("Usage: computor <expression>");
		return;
	}
	let expression: &String = &arg[1];
	let mut parsed_expressions: Vec<EXPR> = Vec::new();

	if let Err(err) = parse(expression, &mut parsed_expressions) {
		eprintln!("{}", err);
		return;
	}

	if let Err(err) = validate(&parsed_expressions) {
		eprintln!("{}", err);
		return;
	}

	println!("{}", degree(&parsed_expressions));

	println!("Expression is valid.");
}

fn main()
{
	computor();
}