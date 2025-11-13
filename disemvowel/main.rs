pub mod disemvowel;

use crate::disemvowel::disemvowel;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // The first command line argument is the name of the executable
    // that was called to run this program. After that should be
    // two command line arguments, both file names. The first should
    // be the name of a file containing the text to disemvowel,
    // and the second should be the file we want to write the disemvoweled text to.
    let args: Vec<String> = env::args().collect();
     

    //If there aren't three arguments, then the program terminates.
    if args.len() != 3
    {
        panic!("Not enough arguments");
    }

    //We create an arbitrary variable s that reads in a file by creating a path of the file we want
    //to read.
    let s = &read_file(Path::new(&args[1]));
    
    //We then call the disemvowel() function using a reference of s. We store the resulting value
    //in s_disemvowel.
    let s_disemvowel = disemvowel(&s);

    //To write the disemvoweled text, we call upon the write_file() function and pass in the file
    //we want to write to along with the disemvoweled text.
    write_file(Path::new(&args[2]), &s_disemvowel);

    //Stop the function.
        
}

fn read_file(path: &Path) -> String {
    fs::read_to_string(path).expect("Could not read the file")
}

fn write_file(path: &Path, s: &str) {
    fs::write(path, s).expect("Unable to write file");
}

// Everything from here down is Rust test code. You shouldn't need to
// change any of this.
//
// Use `cargo test` to run all these tests. All the tests will initially
// fail because there's no definition for the `disemvowel` function. Add
// that up above and work to get the tests to pass. See the lab write-up
// for some tips.

// Tests that check that the correct panics are generated when
// there aren't the correct number of command line arguments
// or the input file isn't readable.
#[cfg(test)]
mod tests {
    use assert_cmd::{assert::OutputAssertExt, cargo::CommandCargoExt};
    use predicates::prelude::predicate;
    use std::process::Command;

    #[test]
    fn requires_two_arguments() {
        // The `Command` library lets you create and run a CLI command.
        let mut cmd = Command::cargo_bin("disemvowel-in-rust").unwrap();
        // Add one command-line argument to this command, namely "1". The assertion
        // below should fail because we didn't provide two arguments as required.
        cmd.arg("1");
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Not enough arguments"));
    }

    #[test]
    fn requires_read_file() {
        let mut cmd = Command::cargo_bin("disemvowel-in-rust").unwrap();
        // Provide two arguments, but both as paths to files that don't actually exist.
        // The assertion below will fail because we couldn't open the first file.
        cmd.arg("/this/path/does/not/exist")
            .arg("output/path/does/not/matter");
        cmd.assert()
            .failure()
            .stderr(predicate::str::contains("Could not read the file"));
    }
}
