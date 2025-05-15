#[cfg(test)]
use std::process::Command;

#[cfg(test)]
fn run_test(input: &str, expected_output: &str) {
	let build_status = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .status()
        .expect("Failed to execute `cargo build --release`");

    assert!(build_status.success(), "Build failed");

	let output = Command::new("target/release/computorv1")
	.arg(input)
	.output()
	.expect("Failed to execute process");

	let stdout = String::from_utf8_lossy(&output.stdout);
	assert_eq!(stdout.trim(), expected_output.trim(), "Test failed for input: {}", input);
}

#[test]
fn test_case_1() {
	run_test(
		"5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0",
		"Reduced form: - 9.3 * X^2 + 4 * X + 4 = 0\nPolynomial degree: 2\nDiscriminant is strictly positive, the two solutions are:\n0.9052389907905898\n-0.47513146390886934"
	);
}

#[test]
fn test_case_2() {
	run_test(
		"5 * X^0 + 4 * X^1 = 4 * X^0",
		"Reduced form: 4 * X + = 0\nPolynomial degree: 1\nSolution : x = -0.25"
	);
}

#[test]
fn test_case_3() {
	run_test(
		"8 * X^0 - 6 * X^1 + 0 * X^2 - 5.6 * X^3 = 3 * X^0",
		"Error: Unsupported polynomial term: '-5.6*X^3'"
	);
}

#[test]
fn test_case_4() {
	run_test(
		"6 * X^0 = 6 * X^0",
		"Reduced form: 0 = 0\nPolynomial degree: 0\nAny real number is a solution."
	);
}

#[test]
fn test_case_5() {
	run_test(
		"10 * X^0 = 15 * X^0",
		"Reduced form: - 5 = 0\nPolynomial degree: 0\nNo solution."
	);
}

#[test]
fn test_case_6() {
	run_test(
		"1 * X^0 + 2 * X^1 + 5 * X^2 = 0",
		"Reduced form: 5 * X^2 + 2 * X + = 0\nPolynomial degree: 2\nDiscriminant is strictly negative, the two complex solutions are:\n-0.2 + 0.4i\n-0.2 - 0.4i"
);
}