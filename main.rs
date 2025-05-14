use std::env;

fn parse(expression: &String) -> Result<(Vec<String>, Vec<String>), String> {
	let clean_expression = expression.replace("-", "+-").replace(" ", "");

	let split: Vec<&str> = clean_expression.split("=").collect();

	if split.len() != 2 {
		return Err("Error: Expression must contain exactly one '=' sign.".to_string());
	}

	let left: Vec<String> = split[0].split("+").map(|s| s.to_string()).collect();
	let right: Vec<String> = split[1].split("+").map(|s| s.to_string()).collect();

	Ok((left, right))
}

fn coefficient(string: String) -> Result<f64, String>
{
	let cleaned_string = string
		.replace("X^2", "")
		.replace("X^1", "")
		.replace("X^0", "")
		.replace("*", "");
	if cleaned_string == "-"
	{
		return Ok(-1.);
	}
	if cleaned_string.is_empty()
	{
		return Ok(1.);
	}
	cleaned_string.parse::<f64>().map_err(|e| e.to_string())
}

fn decompose(left: Vec<String>, right: Vec<String>) -> Result<[f64; 3], String>
{
	let mut a: f64 = 0.0;
	let mut b: f64 = 0.0;
	let mut c: f64 = 0.0;

	for elem in left {
		if elem.find("X^2").is_some() {
			a += match coefficient(elem) {
				Ok(value) => value,
				Err(err) => return Err(err),
			};
		} else if elem.find("X^1").is_some() {
			b += match coefficient(elem) {
				Ok(value) => value,
				Err(err) => return Err(err),
			};
		} else {
			c += match coefficient(elem) {
				Ok(value) => value,
				Err(err) => return Err(err),
			};
		}
	}

	for elem in right {
		if elem.find("X^2").is_some() {
			a -= match coefficient(elem) {
				Ok(value) => value,
				Err(err) => return Err(err),
			};
		} else if elem.find("X^1").is_some() {
			b -= match coefficient(elem) {
				Ok(value) => value,
				Err(err) => return Err(err),
			};
		} else {
			c -= match coefficient(elem) {
				Ok(value) => value,
				Err(err) => return Err(err),
			};
		}
	}

	Ok([a, b, c])
}

fn degree(coeff: [f64; 3]) -> i8
{
	if coeff[0] != 0.
	{
		return 2
	}
	if coeff[1] != 0.
	{
		return 1
	}
	return 0
}

fn solve(a: f64, b: f64, c: f64, degree: i8)
{
	if degree == 0
	{
		if c == 0.
		{
			println!("Any real number is a solution.");
		}
		else
		{
			println!("No solution.");
		}
	}
	else if degree == 1
	{
		println!("Solution : x = {}", -c / b);
	}
	else
	{
		let delta: f64 = b.powf(2.) - 4. * a * c;

		if delta > 0.
		{
			println!("Discriminant is strictly positive, the two solutions are:");
			let x1 = (-b - delta.sqrt()) / (2. * a);
			let x2 = (-b + delta.sqrt()) / (2. * a);
			println!("{}\n{}", x1, x2);
		}
		else if delta == 0.
		{
			println!("The solution is:\n{}", -b / (2. * a));
		}
		else
		{
			println!("Discriminant is strictly negative, the two complex solutions are:");
			let real = -b / (2. * a);
			let imaginary = (-delta).sqrt() / (2. * a);
			println!("{} + {}i\n{} - {}i", real, imaginary, real, imaginary);
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

	let parsed = parse(expression);

	if parsed.is_err() {
		println!("{}", parsed.unwrap_err());
		return;
	}

	let (left, right) = parsed.unwrap();

	let decomposed = decompose(left, right);

	if decomposed.is_err() {
		println!("{}", decomposed.unwrap_err());
		return;
	}

	let coeffs = decomposed.unwrap();

	let degree = degree(coeffs);

	println!("Polynomial degree: {}", degree);
	println!("Coeffs: {:?}", coeffs);
	solve(coeffs[0], coeffs[1], coeffs[2], degree);
}

fn main()
{
	computor();
}