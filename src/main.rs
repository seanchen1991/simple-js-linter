use std::io;
use std::fs;
use std::fmt;
use std::env;
use std::io::Error as E;

// mistake indicating a missing semicolon
struct Mistake {
    path: String,
    text: String,
    locations: Vec<usize>,
}

impl Mistake {
    fn line_bounds(&self, index: usize) -> (usize, usize) {
        let len = self.text.len();

        let before = &self.text[..index];
        let start = before.rfind("\n").map(|x| x + 1).unwrap_or(0);

        let after = &self.text[index + 1..];
        let end = after.find("\n").map(|x| x + index + 1).unwrap_or(len);

        (start, end)
    }
}

impl fmt::Display for Mistake {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for &location in &self.locations {
            let (start, end) = self.line_bounds(location);
            let line = &self.text[start..end];

            let line_number = self.text[..start].matches("\n").count() + 1;
            let comma_index = location - start;

            write!(f, "{}: commas are forbidden:\n\n", self.path)?;

            // print the line, with the line number 
            write!(f, "{:>8} | {}\n", line_number, line)?;

            // indicate where the comma is
            write!(f, "{}^\n\n", " ".repeat(11 + comma_index))?;
        }

        Ok(())
    }
}

fn report(result: Option<Mistake>) {
    if let Some(mistake) = result {
        println!("{}", mistake);
    }
}

// checks the line and determines if it should have a ; at the end 
// if it should, returns an option with the index where the ; should be
fn should_have_semicolon(line: &str) -> Option<usize> {
    let line = line.trim_end();
    let len = line.len();

    let char = line.chars().last().expect("No last char");

    // this will need to be updated to check if the 
    // { or } belongs to a for or if statement 
    let rv = match char as u8 {
        b';' | b'{' | b'}' => None,
        _ => Some(len),
    };

    rv
}

fn lint(path: String) -> Result<Option<Mistake>, E> {
    let text = fs::read_to_string(&path)?;
    
    // check for newlines not preceded by a ;
    let locations: Vec<_> = text
        .lines()
        .filter_map(|line| should_have_semicolon(line))
        .collect();

    Ok(if locations.is_empty() {
        None
    } else {
        Some(Mistake {
            path,
            text,
            locations,
        })
    })
}

fn main() -> Result<(), io::Error> {
    // read in the name of the file from stdin
    let filename = env::args().nth(0).expect("No filename given");

    // for now, we'll only allow the ability to lint a single
    // input file at a time 
    let result = lint(filename)?;
    
    // report the result of linting the file
    report(result);

    Ok(())
}




