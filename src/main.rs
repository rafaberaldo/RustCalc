use std::io;
use regex::Regex;
use eval::eval;

fn parse(input: String) -> Result<String, &'static str> {
	if input.len() == 0 {
		return Err("No calculation provided");
	}

	let open_parenthesis = Regex::new(r"\(").unwrap().find_iter(&input).count();
	let close_parenthesis = Regex::new(r"\)").unwrap().find_iter(&input).count();
	if open_parenthesis != close_parenthesis {
		return Err("Maybe you forgot a parenthesis?");
	}

	// Sanitize the input
	let re = Regex::new(r"[\d+\-*/()]").unwrap();
	let list: Vec<&str> = re.find_iter(&input).map(|m| m.as_str()).collect();
	let calc = list.join("");

	match eval(&calc) {
		Ok(r) => Ok(r.to_string()),
		Err(_) => Err("Invalid formula, couldn't parse it")
	}
}

fn main() {
	let mut input = String::new();
	let stdin = io::stdin();

	println!("\nEnter a calculation:");
  stdin.read_line(&mut input).expect("error: unable to read user input");

	let result = parse(input.clone());
	match result {
			Ok(r) => { println!("Result: {}", r); }
			Err(e) => { println!("{}", e); }
	}

	main();
}

#[test]
fn check_parsing() {
    assert_eq!(parse("12+4".to_string()).unwrap(), "16");
    assert_eq!(parse("12+4*2".to_string()).unwrap(), "20");
    assert_eq!(parse("(12+4)*2".to_string()).unwrap(), "32");
		// parse should remove the `min()` function
    assert_eq!(parse("lmao(12+4)*2*min(2,4)".to_string()).unwrap(), "768");
}
