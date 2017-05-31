use std::fs::File;
use std::io::prelude::*;

extern crate rayon;
use rayon::prelude::*;

fn count_word(line: &str) -> isize {
	let mut nb_word_found = 0;
	let mut last_char_was_word = false;
	for c in line.chars() {
		let char_is_word = c.is_alphanumeric();
		if char_is_word && !last_char_was_word {
			nb_word_found += 1;
//			write!( std::io::stdout(), "{}", '\n' ).unwrap();
		}
		last_char_was_word = char_is_word;
//		if char_is_word {
//			write!( std::io::stdout(), "{}", c ).unwrap();
//		}
	}
	nb_word_found
}

fn run_app() -> Result<(), std::io::Error> {
	let path = "Cargo.toml";
	let file = File::open(path);
	let mut file = match file {
		Ok(file) => file,
		Err(err) => return Err(err), // Put nicer error message with path that was not found
	};
	let mut contents = String::new();
	match file.read_to_string(&mut contents) {
		Ok(_) => (),
		Err(err) => return Err(err),
	};

	let total_word_count: Result<isize, std::io::Error> = contents.par_lines().map( 
		|line| Ok(count_word( line ))
		).sum();
	match total_word_count {
		Ok(count) => println!("{}", count),
		Err(err) => return Err(err),
	};
	
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
