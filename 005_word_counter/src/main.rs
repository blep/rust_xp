use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn run_app() -> Result<(), std::io::Error> {
	let path = "Cargo.toml";
	let file = File::open(path);
	let file = match file {
		Ok(file) => file,
		Err(err) => return Err(err), // Put nicer error message with path that was not found
	};
	let reader = BufReader::new(file);
	for line in reader.lines() {
		match line {
			Ok(line) => println!("Line: {}", line),
			Err(err) => return Err(err),
		};
		
	}
    Ok(())
}

fn main() {
    ::std::process::exit(match run_app() {
       Ok(_) => 0,
       Err(err) => {
           writeln!(std::io::stderr(), "error: {:?}", err).unwrap();
           1
       }
    });
}
